mod test_utils;

fn solve(id: &str) -> f32 {
    let id = format!("Langevin/{}", id);
    test_utils::solve(&id, Some(10000), Some(1))
}


#[test]
fn n20ft301_dat() {
    assert_eq!(661.60, solve("N20ft301.dat"));
}


#[test]
fn n20ft302_dat() {
    assert_eq!(703.00, solve("N20ft302.dat"));
}


#[test]
fn n20ft303_dat() {
    assert_eq!(746.40, solve("N20ft303.dat"));
}


#[test]
fn n20ft304_dat() {
    assert_eq!(817.00, solve("N20ft304.dat"));
}


#[test]
fn n20ft305_dat() {
    assert_eq!(724.70, solve("N20ft305.dat"));
}


#[test]
fn n20ft306_dat() {
    assert_eq!(729.50, solve("N20ft306.dat"));
}


#[test]
fn n20ft307_dat() {
    assert_eq!(691.80, solve("N20ft307.dat"));
}


#[test]
fn n20ft308_dat() {
    assert_eq!(788.20, solve("N20ft308.dat"));
}


#[test]
fn n20ft309_dat() {
    assert_eq!(751.80, solve("N20ft309.dat"));
}


#[test]
fn n20ft310_dat() {
    assert_eq!(693.80, solve("N20ft310.dat"));
}


#[test]
fn n20ft401_dat() {
    assert_eq!(660.90, solve("N20ft401.dat"));
}


#[test]
fn n20ft402_dat() {
    assert_eq!(701.00, solve("N20ft402.dat"));
}


#[test]
fn n20ft403_dat() {
    assert_eq!(746.40, solve("N20ft403.dat"));
}


#[test]
fn n20ft404_dat() {
    assert_eq!(817.00, solve("N20ft404.dat"));
}


#[test]
fn n20ft405_dat() {
    assert_eq!(724.70, solve("N20ft405.dat"));
}


#[test]
fn n20ft406_dat() {
    assert_eq!(728.50, solve("N20ft406.dat"));
}


#[test]
fn n20ft407_dat() {
    assert_eq!(691.80, solve("N20ft407.dat"));
}


#[test]
fn n20ft408_dat() {
    assert_eq!(786.10, solve("N20ft408.dat"));
}


#[test]
fn n20ft409_dat() {
    assert_eq!(749.80, solve("N20ft409.dat"));
}


#[test]
fn n20ft410_dat() {
    assert_eq!(693.80, solve("N20ft410.dat"));
}


#[test]
fn n40ft201_dat() {
    assert_eq!(1109.30, solve("N40ft201.dat"));
}


#[test]
fn n40ft202_dat() {
    assert_eq!(1017.40, solve("N40ft202.dat"));
}


#[test]
fn n40ft203_dat() {
    assert_eq!(903.10, solve("N40ft203.dat"));
}


#[test]
fn n40ft204_dat() {
    assert_eq!(897.40, solve("N40ft204.dat"));
}


#[test]
fn n40ft205_dat() {
    assert_eq!(983.60, solve("N40ft205.dat"));
}


#[test]
fn n40ft206_dat() {
    assert_eq!(1081.90, solve("N40ft206.dat"));
}


#[test]
fn n40ft207_dat() {
    assert_eq!(884.90, solve("N40ft207.dat"));
}


#[test]
fn n40ft208_dat() {
    assert_eq!(1051.60, solve("N40ft208.dat"));
}


#[test]
fn n40ft209_dat() {
    assert_eq!(1027.50, solve("N40ft209.dat"));
}


#[test]
fn n40ft210_dat() {
    assert_eq!(1035.30, solve("N40ft210.dat"));
}


#[test]
fn n40ft401_dat() {
    assert_eq!(1105.20, solve("N40ft401.dat"));
}


#[test]
fn n40ft402_dat() {
    assert_eq!(1016.40, solve("N40ft402.dat"));
}


#[test]
fn n40ft403_dat() {
    assert_eq!(903.10, solve("N40ft403.dat"));
}


#[test]
fn n40ft404_dat() {
    assert_eq!(897.40, solve("N40ft404.dat"));
}


#[test]
fn n40ft405_dat() {
    assert_eq!(982.60, solve("N40ft405.dat"));
}


#[test]
fn n40ft406_dat() {
    assert_eq!(1081.90, solve("N40ft406.dat"));
}


#[test]
fn n40ft407_dat() {
    assert_eq!(872.20, solve("N40ft407.dat"));
}


#[test]
fn n40ft408_dat() {
    assert_eq!(1043.50, solve("N40ft408.dat"));
}


#[test]
fn n40ft409_dat() {
    assert_eq!(1025.50, solve("N40ft409.dat"));
}


#[test]
fn n40ft410_dat() {
    assert_eq!(1034.30, solve("N40ft410.dat"));
}


#[test]
fn n60ft201_dat() {
    assert_eq!(1375.40, solve("N60ft201.dat"));
}


#[test]
fn n60ft202_dat() {
    assert_eq!(1186.40, solve("N60ft202.dat"));
}


#[test]
fn n60ft203_dat() {
    assert_eq!(1194.20, solve("N60ft203.dat"));
}


#[test]
fn n60ft204_dat() {
    assert_eq!(1283.60, solve("N60ft204.dat"));
}


#[test]
fn n60ft205_dat() {
    assert_eq!(1215.50, solve("N60ft205.dat"));
}


#[test]
fn n60ft206_dat() {
    assert_eq!(1238.80, solve("N60ft206.dat"));
}


#[test]
fn n60ft207_dat() {
    assert_eq!(1305.30, solve("N60ft207.dat"));
}


#[test]
fn n60ft208_dat() {
    assert_eq!(1172.60, solve("N60ft208.dat"));
}


#[test]
fn n60ft209_dat() {
    assert_eq!(1243.80, solve("N60ft209.dat"));
}


#[test]
fn n60ft210_dat() {
    assert_eq!(1273.20, solve("N60ft210.dat"));
}


#[test]
fn n60ft301_dat() {
    assert_eq!(1375.40, solve("N60ft301.dat"));
}


#[test]
fn n60ft302_dat() {
    assert_eq!(1184.40, solve("N60ft302.dat"));
}


#[test]
fn n60ft303_dat() {
    assert_eq!(1194.20, solve("N60ft303.dat"));
}


#[test]
fn n60ft304_dat() {
    assert_eq!(1283.60, solve("N60ft304.dat"));
}


#[test]
fn n60ft305_dat() {
    assert_eq!(1214.50, solve("N60ft305.dat"));
}


#[test]
fn n60ft306_dat() {
    assert_eq!(1237.80, solve("N60ft306.dat"));
}


#[test]
fn n60ft307_dat() {
    assert_eq!(1298.40, solve("N60ft307.dat"));
}


#[test]
fn n60ft308_dat() {
    assert_eq!(1168.80, solve("N60ft308.dat"));
}


#[test]
fn n60ft309_dat() {
    assert_eq!(1242.80, solve("N60ft309.dat"));
}


#[test]
fn n60ft310_dat() {
    assert_eq!(1273.20, solve("N60ft310.dat"));
}


#[test]
fn n60ft401_dat() {
    assert_eq!(1375.40, solve("N60ft401.dat"));
}


#[test]
fn n60ft402_dat() {
    assert_eq!(1183.40, solve("N60ft402.dat"));
}


#[test]
fn n60ft403_dat() {
    assert_eq!(1194.20, solve("N60ft403.dat"));
}


#[test]
fn n60ft404_dat() {
    assert_eq!(1283.60, solve("N60ft404.dat"));
}


#[test]
fn n60ft405_dat() {
    assert_eq!(1212.50, solve("N60ft405.dat"));
}


#[test]
fn n60ft406_dat() {
    assert_eq!(1236.80, solve("N60ft406.dat"));
}


#[test]
fn n60ft407_dat() {
    assert_eq!(1296.40, solve("N60ft407.dat"));
}


#[test]
fn n60ft408_dat() {
    assert_eq!(1150.00, solve("N60ft408.dat"));
}


#[test]
fn n60ft409_dat() {
    assert_eq!(1241.80, solve("N60ft409.dat"));
}


#[test]
fn n60ft410_dat() {
    assert_eq!(1273.20, solve("N60ft410.dat"));
}


