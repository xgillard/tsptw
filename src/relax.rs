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
                min_i = min_i.min(pb.instance.distances[(i,j)]);
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
        let mut earliest  = usize::max_value();
        let mut latest    = usize::min_value();

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
            }
            can_visit |= &state.can_visit;
        }

        State {
            position: Position::Virtual(position),
            elapsed : ElapsedTime::FuzzyAmount{earliest, latest},
            can_visit
        }
    }

    fn relax_edge(&self, _: &State, _: &State, _: &State, _: ddo::Decision, cost: isize) -> isize {
        cost
    }

    fn estimate(&self, state  : &State) -> isize {
       let mut total = 0;

       for i in BitSetIter::new(&state.can_visit) {
           total += self.cheapest_edge[i];
       }

       -(total as isize)
    }
    // TODO An example RUB could be the weight of the spanning tree connecting 
    //      the remaining nodes (... but that would be quite expensive to compute
    //      at each node).
    //
    //      Another idea would be to simply consider the negated cost of the 
    //      most expensive edge in the complete matrix and multiply that value
    //      by the number of remaining nodes.
}
