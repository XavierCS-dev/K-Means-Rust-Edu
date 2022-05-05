use crate::calc_dist::calc_dist;
use crate::DIMENSION_OF_POINTS;

pub fn within_cluster_variance(cluster: &Vec<[f64; DIMENSION_OF_POINTS]>, centroid: &[f64]) -> f64 {
    let mut distances:f64 = 0.0;
    for point in cluster {
        let val = calc_dist(point, centroid);
        distances += val * val;
    }

    return distances
}