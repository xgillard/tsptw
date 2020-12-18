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

#[derive(Clone)]
pub struct TSPTWRelax<'a> {
    pb : &'a TSPTW,
    cheapest_edge: Vec<usize>
}
impl <'a> TSPTWRelax<'a> {
    pub fn new(pb: &'a TSPTW) -> Self {
        let cheapest_edge = Self::compute_cheapest_edges(pb);
        Self{pb, cheapest_edge}
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

impl Relaxation<State> for TSPTWRelax<'_> {
    fn merge_states(&self, states: &mut dyn Iterator<Item=&State>) -> State {
        let mut position  = BitSet::new(self.pb.instance.nb_nodes as usize);
        let mut can_visit = BitSet::new(self.pb.instance.nb_nodes as usize);
        let mut all_agree = BitSet::new(self.pb.instance.nb_nodes as usize).not();

        let mut earliest  = usize::max_value();
        let mut latest    = usize::min_value();
        let mut max_tol   = 0_16;

        for state in states {
            match &state.position {
                Position::Node(x)     => position.set(*x as usize, true),
                Position::Virtual(xs) => position |= xs,
            };

            match state.elapsed {
                ElapsedTime::FixedAmount{duration} => { 
                    earliest = earliest.min(duration);
                    latest   = latest.max(duration);
                },
                ElapsedTime::FuzzyAmount{earliest: ex, latest: lx} => {
                    earliest = earliest.min(ex);
                    latest   = latest.max(lx);
                }
            };

            all_agree &= &state.can_visit;
            can_visit |= &state.can_visit;

            max_tol = max_tol.max(state.tolerance);
        }

        let tolerance = can_visit.count_ones() - all_agree.count_ones();
        let tolerance = tolerance.max(max_tol as u32) as u16;

        State {
            position: Position::Virtual(position),
            elapsed : ElapsedTime::FuzzyAmount{earliest, latest},
            can_visit,
            tolerance,
        }
    }

    fn relax_edge(&self, _: &State, _: &State, _: &State, _: ddo::Decision, cost: isize) -> isize {
        cost
    }


    fn estimate(&self, state  : &State) -> isize {
       let must_visit        = (state.can_visit.count_ones() - state.tolerance as u32) as usize;
       let mut cheap         = vec![];
       let mut mandatory     = 0;
       let mut back_to_depot = usize::max_value();
       let mut violations    = 0_u16;


       for i in BitSetIter::new(&state.can_visit) {
           cheap.push(i);
           back_to_depot = back_to_depot.min(self.pb.instance.distances[(i, 0)]);

           let latest   = self.pb.instance.timewindows[i].latest;
           let earliest = state.elapsed.add(self.cheapest_edge[i]).earliest();
           if earliest > latest {
               violations += 1;
           }
       }  

       if violations > state.tolerance {
           return isize::min_value();
       }

       cheap.sort_unstable_by_key(|x| self.cheapest_edge[*x]);
       for x in cheap.iter().take(must_visit) {
           mandatory += self.cheapest_edge[*x];
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
