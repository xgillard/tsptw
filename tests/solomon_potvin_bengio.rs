mod test_utils;

fn solve(id: &str) -> f32 {
    let id = format!("SolomonPotvinBengio/{}", id);
    test_utils::solve(&id, Some(10), Some(1))
}

// The solutions to the instances are to be found in the below link.
// However, they have been rounded a little bit too much so they must be 
// recomputed (but at least the values give an idea)
// http://lopez-ibanez.eu/files/TSPTW/SolomonPotvinBengio-best-known-makespan.txt

#[test]
fn rc_201_1_txt() {
    assert_eq!(592.0611, solve("rc_201.1.txt"));
}


#[test]
fn rc_201_2_txt() {
    assert_eq!(860.1748, solve("rc_201.2.txt"));
}


#[test]
fn rc_201_3_txt() {
    assert_eq!(853.7075, solve("rc_201.3.txt"));
}


#[test]
fn rc_201_4_txt() {
    assert_eq!(889.1761, solve("rc_201.4.txt"));
}


#[test] #[ignore]
fn rc_202_1_txt() {
    assert_eq!(850.48, solve("rc_202.1.txt"));
}


#[test]
fn rc_202_2_txt() {
    assert_eq!(338.5183, solve("rc_202.2.txt"));
}


#[test]
fn rc_202_3_txt() {
    assert_eq!(894.1028, solve("rc_202.3.txt"));
}


#[test] #[ignore]
fn rc_202_4_txt() {
    assert_eq!(853.7075, solve("rc_202.4.txt"));
}


#[test]
fn rc_203_1_txt() {
    assert_eq!(488.4224, solve("rc_203.1.txt"));
}


#[test]
fn rc_203_2_txt() {
    assert_eq!(853.7075, solve("rc_203.2.txt"));
}


#[test]
fn rc_203_3_txt() {
    assert_eq!(921.4397, solve("rc_203.3.txt"));
}


#[test]
fn rc_203_4_txt() {
    assert_eq!(338.5183, solve("rc_203.4.txt"));
}


#[test]
fn rc_204_1_txt() {
    assert_eq!(917.83, solve("rc_204.1.txt"));
}


#[test] #[ignore]
fn rc_204_2_txt() {
    assert_eq!(690.06, solve("rc_204.2.txt"));
}


#[test]
fn rc_204_3_txt() {
    assert_eq!(455.0315, solve("rc_204.3.txt"));
}


#[test]
fn rc_205_1_txt() {
    assert_eq!(417.8058, solve("rc_205.1.txt"));
}


#[test]
fn rc_205_2_txt() {
    assert_eq!(820.1853, solve("rc_205.2.txt"));
}


#[test]
fn rc_205_3_txt() {
    assert_eq!(950.0539, solve("rc_205.3.txt"));
}


#[test]
fn rc_205_4_txt() {
    assert_eq!(837.7083, solve("rc_205.4.txt"));
}


#[test]
fn rc_206_1_txt() {
    assert_eq!(117.8479, solve("rc_206.1.txt"));
}


#[test]
fn rc_206_2_txt() {
    assert_eq!(870.4875, solve("rc_206.2.txt"));
}


#[test]
fn rc_206_3_txt() {
    assert_eq!(650.5942, solve("rc_206.3.txt"));
}


#[test]
fn rc_206_4_txt() {
    assert_eq!(911.98, solve("rc_206.4.txt"));
}


#[test]
fn rc_207_1_txt() {
    assert_eq!(804.6735, solve("rc_207.1.txt"));
}


#[test] #[ignore]
fn rc_207_2_txt() {
    assert_eq!(713.90, solve("rc_207.2.txt"));
}


#[test]
fn rc_207_3_txt() {
    assert_eq!(745.77, solve("rc_207.3.txt"));
}


#[test]
fn rc_207_4_txt() {
    assert_eq!(133.1421, solve("rc_207.4.txt"));
}


#[test]
fn rc_208_1_txt() {
    assert_eq!(810.70, solve("rc_208.1.txt"));
}


#[test]
fn rc_208_2_txt() {
    assert_eq!(579.51, solve("rc_208.2.txt"));
}


#[test]
fn rc_208_3_txt() {
    assert_eq!(686.80, solve("rc_208.3.txt"));
}

