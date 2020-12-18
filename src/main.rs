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

use std::{fs::File, path::Path, time::{Duration, Instant}};

use ddo::{Completion, NbUnassignedWitdh, NoDupFrontier, ParallelSolver, Problem, Solution, Solver, TimeBudget, Times, config_builder};
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
    /// The maximum width of an mdd layer. The value you provide to this 
    /// argument will serve as a multiplicator to the default. Hence, 
    /// providing an argument value `width == 5` for an instance having 20 
    /// "cities" to visit, means that the maximum layer width will be 100.
    /// By default, the number of nodes equates to the number of unassigned
    /// variables.
    #[structopt(name="width", short, long)]
    width: Option<usize>,
    /// How many threads do you want to use to solve the problem ?
    #[structopt(name="threads", short, long)]
    threads: Option<usize>,
    /// How long do you want the solver to keep working on your problem ? (in seconds)
    #[structopt(name="duration", short, long)]
    duration: Option<u64>,
    /// Shall we print the header
    #[structopt(name="header", long)]
    header : bool,
}

fn main() -> Result<(), std::io::Error> {
    let args     = Args::from_args();

    let inst     = TSPTWInstance::from(File::open(&args.instance)?);
    let pb       = TSPTW::new(inst);
    let relax    = TSPTWRelax::new(&pb);
    let mut solvr= mk_solver(&pb, relax, &args);

    let start    = Instant::now();
    let outcome  = solvr.as_mut().maximize();
    let finish   = Instant::now();

    let instance = instance_name(&args.instance);
    let nb_vars  = pb.nb_vars();
    let lb       = objective(solvr.as_ref().best_lower_bound());
    let ub       = objective(solvr.as_ref().best_upper_bound());
    let solution = solvr.as_ref().best_solution();
    let duration = finish - start;

    if args.header {
        print_header();
    }
    print_solution(&instance, nb_vars, outcome, &lb, &ub, duration, solution);
    Ok(())
}
fn print_header() {
    println!("INSTANCE\tSTATUS\t\tUB\t\tLB\t\tDURATION (seconds)\tSOLUTION");
}
fn print_solution(name: &str, n: usize, completion: Completion, lb: &str, ub: &str, duration: Duration, solution: Option<Solution>) {
    println!("{}\t{}\t\t{}\t\t{}\t\t{:18.3}\t{}",
             name, 
             status(completion), 
             lb, ub,
             duration.as_secs_f32(),
             solution_to_string(n, solution));
}
fn instance_name<P: AsRef<Path>>(name: P) -> String {
    name.as_ref().file_stem().unwrap().to_str().unwrap().to_string()
}
fn objective(x: isize) -> String {
    match x {
        isize::MIN => "+inf".to_string(),
        isize::MAX => "-inf".to_string(),
        _ => format!("{:.2}", -(x as f32 / 10_000.0_f32))
    }
}
fn status(completion: Completion) -> &'static str {
   if completion.is_exact {
       "Proved"
   } else {
       "Timeout"
   }
}
fn solution_to_string(nb_vars: usize, solution: Option<Solution>) -> String {
    match solution {
        None   => "No feasible solution found".to_string(),
        Some(s)=> {
           let mut perm = vec![0; nb_vars];
           for d in s.iter() {
               perm[d.variable.id()] = d.value;
           }
           let mut txt = String::new();
           for v in perm {
                txt = format!("{} {}", txt, v);
           }
           txt
        }
    }
}

fn mk_solver<'a, 'b>(pb: &'a TSPTW, relax: TSPTWRelax<'a>, args: &Args) -> Box<dyn Solver + 'a> {
    match (&args.width, &args.duration) {
        (Some(w), Some(d)) => {
            let mdd = config_builder(pb, relax)
                .with_max_width(Times(*w, NbUnassignedWitdh))
                .with_cutoff(TimeBudget::new(Duration::from_secs(*d)))
                .into_deep();
            let solver = ParallelSolver::new(mdd)
                .with_verbosity(args.verbosity.unwrap_or(0))
                .with_nb_threads(args.threads.unwrap_or(num_cpus::get()))
                .with_frontier(NoDupFrontier::default());
            Box::new(solver)
        },
        (Some(w), None) => {
            let mdd = config_builder(pb, relax)
                .with_max_width(Times(*w, NbUnassignedWitdh))
                .into_deep();
            let solver = ParallelSolver::new(mdd)
                .with_verbosity(args.verbosity.unwrap_or(0))
                .with_nb_threads(args.threads.unwrap_or(num_cpus::get()))
                .with_frontier(NoDupFrontier::default());
            Box::new(solver)
        },
        (None, Some(d)) => {
            let mdd = config_builder(pb, relax)
                .with_cutoff(TimeBudget::new(Duration::from_secs(*d)))
                .into_deep();
            let solver = ParallelSolver::new(mdd)
                .with_verbosity(args.verbosity.unwrap_or(0))
                .with_nb_threads(args.threads.unwrap_or(num_cpus::get()))
                .with_frontier(NoDupFrontier::default());
            Box::new(solver)
        },
        (None, None) => {
            let mdd = config_builder(pb, relax)
                .into_deep();
            let solver = ParallelSolver::new(mdd)
                .with_verbosity(args.verbosity.unwrap_or(0))
                .with_nb_threads(args.threads.unwrap_or(num_cpus::get()))
                .with_frontier(NoDupFrontier::default());
            Box::new(solver)
        }
    }
}
