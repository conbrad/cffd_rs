mod cffdrs;
use cffdrs::cfb::cfb_calc;
use cffdrs::fi::fi_calc;

#[no_mangle]
pub extern "C" fn CFBcalc(
    fmc: &[f64],
    sfc: &[f64],
    ros: &[f64],
    cbh: &[f64],
    option: &str,
) -> Vec<f64> {
    let res = cfb_calc(fmc, sfc, ros, cbh, option);
    let handled_res = match res {
        Ok(cfb) => cfb,
        Err(_error) => vec![0.0; cbh.len()],
    };
    return handled_res;
}

#[no_mangle]
pub extern "C" fn FIcalc(fc: &[f64], ros: &[f64]) -> Vec<f64> {
    let res = fi_calc(fc, ros);
    let handled_res = match res {
        Ok(cfb) => cfb,
        Err(_error) => vec![0.0; fc.len()],
    };
    return handled_res;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
