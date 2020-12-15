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

//! This module contains the definition of the dynamic programming formulation 
//! of the TSP+TW. (Implementation of the `Problem` trait).

use std::ops::Not;

use bitset_fixed::BitSet;
use ddo::{BitSetIter, Domain, Problem};

use crate::{instance::TSPTWInstance, state::{ElapsedTime, Position, State}};


/// This is the structure encapsulating the TSPTW problem.
#[derive(Clone)]
pub struct TSPTW {
    pub instance: TSPTWInstance,
    pub initial : State,
}
impl TSPTW {
    pub fn new(inst: TSPTWInstance) -> Self {
        let mut state = State {
            position : Position::Node(0),
            elapsed  : ElapsedTime::FixedAmount{duration: 0},
            can_visit: BitSet::new(inst.nb_nodes as usize).not()
        };
        state.can_visit.set(0, false);
        Self { instance: inst, initial: state }
    }
}

impl Problem<State> for TSPTW {
    fn nb_vars(&self) -> usize {
        self.instance.nb_nodes as usize
    }

    fn initial_state(&self) -> State {
        self.initial.clone()
    }

    fn initial_value(&self) -> isize {
        0
    }
    
    fn domain_of<'a>(&self, state: &'a State, var: ddo::Variable) -> ddo::Domain<'a> {
        let mut domain     = vec![];
        
        for i in BitSetIter::new(&state.can_visit) {
            if self.can_move_to(state, i) {
                domain.push(i as isize);
            }
        }

        // It is only allowed to move back to zero to CLOSE the tour
        if var.id() == self.nb_vars() - 1 {
            domain.push(0);
        }
        
        Domain::from(domain)
    }

    fn transition(&self, state: &State, _vars : &ddo::VarSet, d: ddo::Decision) -> State {
        let mut remaining = state.can_visit.clone();
        remaining.set(d.value as usize, false);

        let time = self.arrival_time(state, d.value as usize);

        State {
            position : Position::Node(d.value as u16),
            elapsed  : time,
            can_visit: remaining
        }
    }

    fn transition_cost(&self, state: &State, _vars : &ddo::VarSet, d: ddo::Decision) -> isize {
        // TSPTW is a minimization problem but the solver works with a 
        // maximization perspective. So we have to negate the min if we want to
        // yield a lower bound.
        let twj = self.instance.timewindows[d.value as usize];
        let travel_time = self.min_distance_to(state, d.value as usize);
        let waiting_time = match state.elapsed {
            ElapsedTime::FixedAmount{duration} => 
                if (duration + travel_time) < twj.earliest {
                    twj.earliest - (duration + travel_time)
                } else {
                    0
                },
            ElapsedTime::FuzzyAmount{earliest, ..} => 
                if (earliest + travel_time) < twj.earliest {
                    twj.earliest - (earliest + travel_time)
                } else {
                    0
                }
        };

        -( (travel_time + waiting_time) as isize)
    }
}

impl TSPTW {
    fn can_move_to(&self, state: &State, j: usize) -> bool {
        let twj         = self.instance.timewindows[j];
        let min_arrival = state.elapsed.add(self.min_distance_to(state, j));
        match min_arrival {
            ElapsedTime::FixedAmount{duration}     => duration <= twj.latest,
            ElapsedTime::FuzzyAmount{earliest, ..} => earliest <= twj.latest,
        }
    }
    fn arrival_time(&self, state: &State, j: usize) -> ElapsedTime {
       let min_arrival = state.elapsed.add(self.min_distance_to(state, j));
       let max_arrival = state.elapsed.add(self.max_distance_to(state, j));

       let min_arrival = match min_arrival {
           ElapsedTime::FixedAmount{duration}     => duration,
           ElapsedTime::FuzzyAmount{earliest, ..} => earliest
       };
       let max_arrival = match max_arrival {
           ElapsedTime::FixedAmount{duration}    => duration,
           ElapsedTime::FuzzyAmount{latest, ..}  => latest
       };
       // This would be the arrival time if we never had to wait.
       let arrival_time = 
           if min_arrival.eq(&max_arrival) { 
               ElapsedTime::FixedAmount{duration: min_arrival} 
           } else {
               ElapsedTime::FuzzyAmount{earliest: min_arrival, latest: max_arrival}
           };
       // In order to account for the possible waiting time, we need to adjust
       // the earliest arrival time
       let twj = self.instance.timewindows[j];
       let arrival_time = 
           match arrival_time {
              ElapsedTime::FixedAmount{duration} => {
                  ElapsedTime::FixedAmount{duration: duration.max(twj.earliest)}
              },
              ElapsedTime::FuzzyAmount{mut earliest, mut latest} => {
                earliest = earliest.max(twj.earliest);
                latest   = latest.min(twj.latest);

                if earliest.eq(&latest) {
                    ElapsedTime::FixedAmount{duration: earliest}
                } else {
                    ElapsedTime::FuzzyAmount{earliest, latest}
                }
              },
          };

       arrival_time
    }
    fn min_distance_to(&self, state: &State, j: usize) -> usize {
        match &state.position {
            Position::Node(i) => self.instance.distances[(*i as usize, j)],
            Position::Virtual(candidates) => 
                BitSetIter::new(candidates)
                    .map(|i| self.instance.distances[(i as usize, j as usize)])
                    .min()
                    .unwrap()
        }
    }
    fn max_distance_to(&self, state: &State, j: usize) -> usize {
        match &state.position {
            Position::Node(i) => self.instance.distances[(*i as usize, j)],
            Position::Virtual(candidates) => 
                BitSetIter::new(candidates)
                    .map(|i| self.instance.distances[(i as usize, j as usize)])
                    .max()
                    .unwrap()
        }
    }
}
