// Copyright 2020 Xavier Gillard
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! This module contains the definition and implementation of the relaxation 
//! for the TSP + TW problem.

use std::ops::Not;

use bitset_fixed::BitSet;
use ddo::{BitSetIter, Problem, Relaxation};

use crate::{model::TSPTW, state::{ElapsedTime, Position, State}};
use std::cell::RefCell;

#[derive(Clone)]
pub struct TSPTWRelax<'a> {
    pb : &'a TSPTW,
    cheapest_edge: Vec<usize>,
    helper: RefCell<RelaxHelper>,
}
impl <'a> TSPTWRelax<'a> {
    pub fn new(pb: &'a TSPTW) -> Self {
        let cheapest_edge = Self::compute_cheapest_edges(pb);
        let helper = RefCell::new(RelaxHelper::new(pb.nb_vars()));
        Self{pb, cheapest_edge, helper}
    }

    fn compute_cheapest_edges(pb: &'a TSPTW) -> Vec<usize> {
        let mut cheapest = vec![];
        let n = pb.nb_vars();
        for i in 0..n {
            let mut min_i = usize::max_value();
            for j in 0..n {
                if i == j {
                    continue;
                }
                min_i = min_i.min(pb.instance.distances[(j, i)]);
            }
            cheapest.push(min_i);
        }
        cheapest
    }
}
#[derive(Clone)]
struct RelaxHelper {
    depth    : u16,
    position : BitSet,
    earliest : usize,
    latest   : usize,
    all_must : BitSet,
    all_agree: BitSet,
    all_maybe: BitSet,
    temp     : Vec<usize>,
}
impl RelaxHelper {
    fn new(n: usize) -> Self {
        Self {
            depth    : 0_u16,
            position : BitSet::new(n),
            earliest : usize::max_value(),
            latest   : usize::min_value(),
            all_must : BitSet::new(n),
            all_agree: BitSet::new(n).not(),
            all_maybe: BitSet::new(n),
            temp     : vec![],
        }
    }
    fn clear(&mut self) {
        self.depth    = 0_u16;
        self.earliest = usize::max_value();
        self.latest   = usize::min_value();
        self.position .buffer_mut().iter_mut().for_each(|x| *x = 0);
        self.all_must .buffer_mut().iter_mut().for_each(|x| *x = 0);
        self.all_agree.buffer_mut().iter_mut().for_each(|x| *x = u64::max_value());
        self.all_maybe.buffer_mut().iter_mut().for_each(|x| *x = 0);
        //self.temp.clear();
    }
    fn track_depth(&mut self, depth: u16) {
        self.depth = self.depth.max(depth);
    }
    fn track_position(&mut self, pos: &Position) {
        match pos {
            Position::Node(x)     => self.position.set(*x as usize, true),
            Position::Virtual(xs) => self.position |= xs,
        };
    }
    fn track_elapsed(&mut self, elapsed: ElapsedTime) {
        match elapsed {
            ElapsedTime::FixedAmount{duration} => {
                self.earliest = self.earliest.min(duration);
                self.latest   = self.latest.max(duration);
            },
            ElapsedTime::FuzzyAmount{earliest: ex, latest: lx} => {
                self.earliest = self.earliest.min(ex);
                self.latest   = self.latest.max(lx);
            }
        };
    }
    fn track_must_visit(&mut self, bs: &BitSet) {
        self.all_agree &= bs;
        self.all_must  |= bs;
    }
    fn track_maybe(&mut self, bs: &Option<BitSet>) {
        if let Some(bs) = bs.as_ref() {
            self.all_maybe |= bs;
        }
    }

    fn get_depth(&self) -> u16 {
        self.depth
    }
    fn get_position(&self) -> Position {
        Position::Virtual(self.position.clone())
    }
    fn get_elapsed(&self) -> ElapsedTime {
        if self.earliest == self.latest {
            ElapsedTime::FixedAmount {duration: self.earliest}
        } else {
            ElapsedTime::FuzzyAmount {earliest: self.earliest, latest: self.latest}
        }
    }
    fn get_must_visit(&self) -> BitSet {
        self.all_agree.clone()
    }
    fn get_maybe_visit(&self)-> Option<BitSet> {
        let mut maybe = self.all_maybe.clone(); // three lines: faster because it is in-place
        maybe |= &self.all_must;
        maybe ^= &self.all_agree;

        let count = maybe.count_ones();
        if count > 0 {
            Some(maybe)
        } else {
            None
        }
    }
}

impl Relaxation<State> for TSPTWRelax<'_> {
    fn merge_states(&self, states: &mut dyn Iterator<Item=&State>) -> State {
        let mut helper = self.helper.borrow_mut();
        helper.clear();

        for state in states {
            helper.track_depth(state.depth);
            helper.track_position(&state.position);
            helper.track_elapsed(state.elapsed);
            helper.track_must_visit(&state.must_visit);
            helper.track_maybe(&state.maybe_visit);
        }

        State {
            depth      : helper.get_depth(),
            position   : helper.get_position(),
            elapsed    : helper.get_elapsed(),
            must_visit : helper.get_must_visit(),
            maybe_visit: helper.get_maybe_visit(),
        }
    }

    fn relax_edge(&self, _: &State, _: &State, _: &State, _: ddo::Decision, cost: isize) -> isize {
        cost
    }


    fn estimate(&self, state  : &State) -> isize {
       let mut complete_tour = self.pb.nb_vars() - state.depth as usize;

       let mut helper        = self.helper.borrow_mut(); 
       let mut mandatory     = 0;
       let mut back_to_depot = usize::max_value();
       
       helper.temp.clear();

       for i in BitSetIter::new(&state.must_visit) {
           complete_tour -= 1;
           mandatory += self.cheapest_edge[i];
           back_to_depot = back_to_depot.min(self.pb.instance.distances[(i, 0)]);

           let latest   = self.pb.instance.timewindows[i].latest;
           let earliest = state.elapsed.add(self.cheapest_edge[i]).earliest();
           if earliest > latest {
               return isize::min_value();
           }
       }

       if let Some(maybes) = state.maybe_visit.as_ref() {
            let mut violations = 0;

            for i in BitSetIter::new(maybes) {
               helper.temp.push(self.cheapest_edge[i]);
               back_to_depot = back_to_depot.min(self.pb.instance.distances[(i, 0)]);
            
               let latest   = self.pb.instance.timewindows[i].latest;
               let earliest = state.elapsed.add(self.cheapest_edge[i]).earliest();
               if earliest > latest {
                   violations += 1;
               }
            }

            if helper.temp.len() - violations < complete_tour {
                return isize::min_value();
            }

            helper.temp.sort_unstable();
            mandatory += helper.temp.iter().copied().take(complete_tour).sum::<usize>();
       }

       // When there is no other city that MUST be visited, we must consider 
       // the shortest distance between *here* (current position) and the 
       // depot.
       if mandatory == 0 {
           back_to_depot = back_to_depot.min(
               match &state.position {
                Position::Node(x) => 
                    self.pb.instance.distances[(*x as usize, 0)],
                Position::Virtual(bs) =>
                    BitSetIter::new(bs).map(|x| self.pb.instance.distances[(x, 0)]).min().unwrap()
           });
       }

       // When it is impossible to get back to the depot in time, the current
       // state is infeasible. So we can give it an infinitely negative ub.
       let total_distance  = mandatory + back_to_depot;
       let earliest_arrival= state.elapsed.add(total_distance).earliest();
       let latest_deadline = self.pb.instance.timewindows[0].latest;
       if earliest_arrival > latest_deadline {
           isize::min_value()
       } else {
            -(total_distance as isize)
       }
    }


    // TODO An example RUB could be the weight of the spanning tree connecting 
    //      the remaining nodes (... but that would be quite expensive to compute
    //      at each node).
    //
    //      Another idea would be to simply consider the negated cost of the 
    //      most expensive edge in the complete matrix and multiply that value
    //      by the number of remaining nodes.
}
