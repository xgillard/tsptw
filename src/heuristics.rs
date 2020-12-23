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

//! This module provides the implementation for some heuristics that may be used
//! to improve the behavior of the branch-and-bound-MDD solver for travelling
//! salesman problem with time windows.

use ddo::{FrontierNode, Problem, LoadVars, Variable, VarSet, WidthHeuristic};

use crate::state::State;
use crate::model::TSPTW;

#[derive(Debug, Copy, Clone)]
pub struct LoadVarsFromDepth{
    nb_vars: usize,
}
impl LoadVarsFromDepth {
    pub fn new(pb: &TSPTW) -> Self {
        Self {nb_vars: pb.nb_vars()}
    }
}
impl LoadVars<State> for LoadVarsFromDepth {
    #[inline]
    fn variables(&self, node: &FrontierNode<State>) -> VarSet {
        let depth   = node.state.depth;
        let mut ret = VarSet::all(self.nb_vars);

        for i in 0..depth {
            ret.remove(Variable(i as usize));
        }
        ret
    }
}


#[derive(Debug, Copy, Clone)]
pub struct IncreasingWithDepth{
    nb_vars: usize,
}
impl IncreasingWithDepth {
    pub fn new(pb: &TSPTW) -> Self {
        Self {nb_vars: pb.nb_vars()}
    }
}
impl WidthHeuristic for IncreasingWithDepth {
    #[inline]
    fn max_width(&self, free_vars: &VarSet) -> usize {
        let depth = self.nb_vars - free_vars.len();
        let factor= 1 + depth;
        factor * self.nb_vars
    }
}
