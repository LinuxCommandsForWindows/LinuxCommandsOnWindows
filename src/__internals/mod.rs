pub fn __InternalsToUnits(value: f64) -> String {
    return if value > 1024f64.powi(3) {
        format!("{:.2} TB", value / 1024.0f64.powi(3))
    }
    else {
        format!("{:.2} MB", value / 1024.0f64.powi(2))
    }
}
