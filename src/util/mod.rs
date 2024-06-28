/** dBFS => 원래 값으로 변환 */
pub fn from_dbfs(dbfs: f64) -> f64 {
    return 10.0_f64.powf(dbfs / 20.0);
}

/** 원래 값 => dBFS로 변환 */
pub fn to_dbfs(val: f64) -> f64 {
    return 20.0 * val.log10();
}