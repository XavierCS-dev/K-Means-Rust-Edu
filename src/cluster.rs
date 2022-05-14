use crate::{DIMENSION_OF_POINTS, NUMBER_OF_CLUSTERS};

#[derive(Default)]
pub struct Cluster {
    pub points: Vec<[f64; DIMENSION_OF_POINTS]>,
    pub centroid: [f64;DIMENSION_OF_POINTS],
    pub old_centroid: [f64;DIMENSION_OF_POINTS],
    pub within_cluster_variance: f64,
}

impl Cluster {
    pub fn initialise_clusters(centroids: [[f64;DIMENSION_OF_POINTS]; NUMBER_OF_CLUSTERS])
                           -> [Cluster; NUMBER_OF_CLUSTERS] {
        let mut clusters: [Cluster; NUMBER_OF_CLUSTERS] = Default::default();
        for i in 0..NUMBER_OF_CLUSTERS {
            clusters[i] = Cluster {
                points: Default::default(),
                centroid: centroids[i],
                old_centroid: centroids[i],
                within_cluster_variance: 0.0,
            }
        }
        return clusters
    }

    pub fn reset_points(&mut self) {
        self.points = Default::default();
    }
}