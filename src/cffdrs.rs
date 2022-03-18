pub fn cfb_calc(
    fmc: &[f64],
    sfc: &[f64],
    ros: &[f64],
    cbh: &[f64],
    option: &str,
) -> Result<Vec<f64>, f64> {
    // Same length guard
    let all_inputs = vec![fmc, sfc, ros, cbh];
    let all_same_length = all_inputs
        .iter()
        .all(|ref v| v.len() == all_inputs[0].len());
    if all_same_length.eq(&false) {
        return Err(-1.0);
    }

    // Eq. 56 (FCFDG 1992) Critical surface intensity
    // CSI = 0.001 * (CBH**1.5) * (460 + 25.9 * FMC)**1.5
    let lhs: Vec<f64> = cbh.iter().map(|x| f64::powf(*x, 1.5) * 0.001).collect();
    let rhs: Vec<f64> = fmc
        .iter()
        .map(|x| f64::powf(x * 25.9 + 460.0, 1.5))
        .collect();
    let csi: Vec<f64> = lhs.iter().zip(rhs.iter()).map(|(x, y)| x * y).collect();

    // Eq. 57 (FCFDG 1992) Surface fire rate of spread (m/min)
    // RSO = CSI / (300 * SFC)
    let rso = csi
        .iter()
        .zip(sfc.iter().map(|x| x * 300.0))
        .map(|(x, y)| x / y)
        .collect();
    if option == "RSO" {
        return Ok(rso);
    }

    // Eq. 58 (FCFDG 1992) Crown fraction burned
    // CFB <- ifelse(ROS > RSO, 1 - exp(-0.23 * (ROS - RSO)), CFB)
    let cfb = rso
        .iter()
        .zip(ros.iter())
        .map(|(x, y)| {
            if y > x {
                1.0 - f64::exp(-0.23) * (y - x)
            } else {
                0.0
            }
        })
        .collect();

    return Ok(cfb);
}
