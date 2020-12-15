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

//! This is the main entry point of the program. This is what gets compiled to
//! the tsptw binary.

use std::fs::File;

use ddo::{NoDupFrontier, ParallelSolver, Problem, Solver, config_builder};
use structopt::StructOpt;
use tsptw::{instance::TSPTWInstance, model::TSPTW, relax::TSPTWRelax};

/// TSPTW is a solver based on branch-and-bound mdd which solves the travelling
/// salesman problem with time windows to optimality. 
///
/// The implementation of tsptw is based on 
/// 'ddo: a generic and efficient framework for MDD-based optimization' (IJCAI20) 
#[derive(StructOpt)]
struct Args {
    /// The path to the TSW+TW instance that needs to be solved.
    instance : String,
    /*
    /// The verbosity level of what is going to be logged on the console.
    verbosity: Option<usize>,
    /// The maximum width of an mdd layer
    width    : Option<usize>,
    /// How many threads do you want to use to solve the problem ?
    threads  : Option<usize>
    */
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::from_args();

    let inst = TSPTWInstance::from(File::open(&args.instance)?);
    let pb   = TSPTW::new(inst);
    let relax= TSPTWRelax::new(&pb);
    let conf = config_builder(&pb, relax);
    let mdd  = conf.into_deep();

    let mut solver = ParallelSolver::new(mdd)
        .with_verbosity(2)
        .with_frontier(NoDupFrontier::default());
    let outcome    = solver.maximize();

    if let Some(best_value) = outcome.best_value {
        let best = -(best_value as f32 / 10000.0);
        println!("The shortest tour is completed in {}", best);

        print!("Permutation : ");
        let mut sln = vec![0; pb.nb_vars()];

        for decision in solver.best_solution().unwrap().iter() {
            sln[decision.variable.id()] = decision.value;
        }
        println!("{:?}", sln);
    } else {
        println!("There is no solution to this problem");
    }

    Ok(())
}
