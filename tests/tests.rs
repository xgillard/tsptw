use std::{fs::File, path::PathBuf};

use ddo::{FixedWidth, NoDupFrontier, ParallelSolver, Solver, config_builder};
use tsptw::{instance::TSPTWInstance, model::TSPTW, relax::TSPTWRelax};



fn locate(id: &str) -> PathBuf {
    PathBuf::new()
        .join(env!("CARGO_MANIFEST_DIR"))
        .join("tests/resources/SolomonPotvinBengio")
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

fn solve(instance: &str, width: Option<usize>, threads: Option<usize>) -> f32 {
   let file       = File::open(locate(instance)).expect("file not found");
   let inst       = TSPTWInstance::from(file);
   let pb         = TSPTW::new(inst);
   let relax      = TSPTWRelax::new(&pb);
   let mut solver = mk_solver(&pb, relax, width, threads);
   let outcome=solver.as_mut().maximize();
   outcome.best_value.map(|v| -(v as f32) / 10000.0).unwrap_or(-1.0)
}

// The solutions to the instances are to be found in the below link.
// However, they have been rounded a little bit too much so they must be 
// recomputed (but at least the values give an idea)
// http://lopez-ibanez.eu/files/TSPTW/SolomonPotvinBengio-best-known-makespan.txt
#[test]
fn rc_201_1() {
    assert_eq!(592.0611, solve("rc_201.1.txt", Some(100), Some(1)));
}
#[test]
fn rc_201_2() {
    assert_eq!(860.1748, solve("rc_201.2.txt", Some(100), Some(1)));
}
#[test]
fn rc_201_3() {
    assert_eq!(853.7075, solve("rc_201.3.txt", Some(100), Some(1)));
}
#[test]
fn rc_201_4() {
    assert_eq!(889.1761, solve("rc_201.4.txt", Some(100), Some(1)));
}
// ---------------------------------------------------------------------------
#[test] #[ignore]  // ignored because it takes long
fn rc_202_1() {
    assert_eq!(850.48, solve("rc_202.1.txt", Some(100), Some(1)));
}
#[test]
fn rc_202_2() {
    assert_eq!(338.5183, solve("rc_202.2.txt", Some(100), Some(1)));
}
#[test]
fn rc_202_3() {
    assert_eq!(894.1028, solve("rc_202.3.txt", Some(100), Some(1)));
}
#[test]
fn rc_202_4() {
    assert_eq!(853.7075, solve("rc_202.4.txt", Some(100), Some(1)));
}
// ---------------------------------------------------------------------------
#[test]
fn rc_203_1() {
    assert_eq!(488.4224, solve("rc_203.1.txt", Some(100), Some(1)));
}
#[test]
fn rc_203_2() {
    assert_eq!(853.7075, solve("rc_203.2.txt", Some(100), Some(1)));
}
#[test] #[ignore] // ignored because it takes long
fn rc_203_3() {
    assert_eq!(921.44, solve("rc_203.3.txt", Some(100), Some(1)));
}
#[test]
fn rc_203_4() {
    assert_eq!(338.5183, solve("rc_203.4.txt", Some(100), Some(1)));
}


