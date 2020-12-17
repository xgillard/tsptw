mod test_utils;

fn solve(id: &str, width: Option<usize>, threads: Option<usize>) -> f32 {
    let id = format!("SolomonPotvinBengio/{}", id);
    test_utils::solve(&id, width, threads)
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
#[test] #[ignore] // ignored because it takes long (105s div. 4 threads)
fn rc_203_3() {
    assert_eq!(921.4397, solve("rc_203.3.txt", Some(100), Some(1)));
}
#[test]
fn rc_203_4() {
    assert_eq!(338.5183, solve("rc_203.4.txt", Some(100), Some(1)));
}


