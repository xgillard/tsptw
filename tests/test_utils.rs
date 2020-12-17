use std::{fs::File, path::PathBuf};

use ddo::{FixedWidth, NoDupFrontier, ParallelSolver, Solver, config_builder};
use tsptw::{instance::TSPTWInstance, model::TSPTW, relax::TSPTWRelax};



fn locate(id: &str) -> PathBuf {
    PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("tests/resources/")
        .join(id)
}


fn mk_solver<'a, 'b>(pb: &'a TSPTW, relax: TSPTWRelax<'a>, width: Option<usize>, threads: Option<usize>) -> Box<dyn Solver + 'a> {
    if let Some(w) = width {
        let mdd = config_builder(pb, relax)
            .with_max_width(FixedWidth(w))
            .into_deep();
        let solver = ParallelSolver::new(mdd)
            .with_nb_threads(threads.unwrap_or(num_cpus::get()))
            .with_frontier(NoDupFrontier::default());
        Box::new(solver)
    } else {
        let conf = config_builder(pb, relax);
        let mdd  = conf.into_deep();
        let solver = ParallelSolver::new(mdd)
            .with_nb_threads(threads.unwrap_or(num_cpus::get()))
            .with_frontier(NoDupFrontier::default());
        Box::new(solver)
    }
}

pub fn solve(instance: &str, width: Option<usize>, threads: Option<usize>) -> f32 {
   let file       = File::open(locate(instance)).expect("file not found");
   let inst       = TSPTWInstance::from(file);
   let pb         = TSPTW::new(inst);
   let relax      = TSPTWRelax::new(&pb);
   let mut solver = mk_solver(&pb, relax, width, threads);
   let outcome=solver.as_mut().maximize();
   outcome.best_value.map(|v| -(v as f32) / 10000.0).unwrap_or(-1.0)
}

