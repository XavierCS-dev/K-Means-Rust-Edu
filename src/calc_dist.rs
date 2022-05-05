
pub fn calc_dist (point_one: &[f64], point_two: &[f64]) -> f64 {
    let mut total: f64 = 0.0;
    for i in 0..point_one.len() {
        total += (point_one[i] - point_two[i]) * (point_one[i] - point_two[i]);
    }
    return total.sqrt()
}

