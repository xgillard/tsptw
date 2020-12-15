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

use ddo::{NoDupFrontier, ParallelSolver, Problem, Solver, FixedWidth, config_builder};
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
    instance: String,
    /// The verbosity level of what is going to be logged on the console.
    #[structopt(name="verbosity", short, long)]
    verbosity: Option<u8>,
    /// The maximum width of an mdd layer
    #[structopt(name="width", short, long)]
    width: Option<usize>,
    /// How many threads do you want to use to solve the problem ?
    #[structopt(name="threads", short, long)]
    threads: Option<usize>
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::from_args();

    let inst = TSPTWInstance::from(File::open(&args.instance)?);
    let pb   = TSPTW::new(inst);
    let relax= TSPTWRelax::new(&pb);
    let mut solvr= mk_solver(&pb, relax, &args);
    let _  = solvr.as_mut().maximize();

    print_solution(pb.nb_vars(), solvr.as_ref());
    Ok(())
}

fn mk_solver<'a, 'b>(pb: &'a TSPTW, relax: TSPTWRelax<'a>, args: &Args) -> Box<dyn Solver + 'a> {
    if let Some(w) = args.width {
        let mdd = config_builder(pb, relax)
            .with_max_width(FixedWidth(w))
            .into_deep();
        let solver = ParallelSolver::new(mdd)
            .with_verbosity(args.verbosity.unwrap_or(0))
            .with_nb_threads(args.threads.unwrap_or(num_cpus::get()))
            .with_frontier(NoDupFrontier::default());
        Box::new(solver)
    } else {
        let conf = config_builder(pb, relax);
        let mdd  = conf.into_deep();
        let solver = ParallelSolver::new(mdd)
            .with_verbosity(args.verbosity.unwrap_or(0))
            .with_nb_threads(args.threads.unwrap_or(num_cpus::get()))
            .with_frontier(NoDupFrontier::default());
        Box::new(solver)
    }
}
fn print_solution(n: usize, solver: &dyn Solver) {
   if let Some(solution) = solver.best_solution() {
       println!("Best cost = {}", -(solver.best_value().unwrap() as f32 / 10000.0));
       let mut perm = vec![0; n];
       for d in solution.iter() {
           perm[d.variable.id()] = d.value;
       }
       for v in perm {
           print!("{} ", v);
       }
       println!();
   } else {
       println!("There is no feasible solution to this problem");
   }
}
