// f64 doesn't implement trait ord so this was necessary
pub fn find_min(distances: &[f64]) -> usize {
    let mut min: f64 = 10000000.0;
    let mut pos: usize = 0;
    let mut min_pos: usize = 0;
    for distance in distances {
        if (*distance).is_normal() {
            if *distance < min {
                min = *distance;
                min_pos = pos;
            }
        }
        pos += 1;
    }
    return min_pos
}