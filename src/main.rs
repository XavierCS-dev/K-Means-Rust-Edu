use crate::cluster::*;
mod calc_dist;
mod find_min;
mod within_cluster_variance;
mod cluster;

// Initialisation constants
pub const NUMBER_OF_CLUSTERS: usize = 2;
pub const NUMBER_OF_POINTS: usize = 8;
pub const DIMENSION_OF_POINTS: usize = 2;

// Points to be classified
const POINT_SET:[[f64; DIMENSION_OF_POINTS]; NUMBER_OF_POINTS] = [[3.0,1.0], [5.0,1.0], [4.0,2.0], [5.0,2.0], [2.0,5.0], [7.0,4.0], [1.0,0.0], [8.0,0.0]];

fn main () {

    // Set of clusters
    // Initial cluster points to be put here (as many points as clusters)
    let mut clusters = Cluster::initialise_clusters([[1.0,2.0], [1.0, 4.0]]);
    // loop iterations
    let mut passes = 0;
    // for storing the distances
    let mut checks: [f64; NUMBER_OF_CLUSTERS] = [0.0; NUMBER_OF_CLUSTERS];
    // for calculating the new mean
    let mut totals: [f64; DIMENSION_OF_POINTS];

    loop {
        passes += 1;
        println!("\npass: {}\n", passes);
        for cluster in &mut clusters {
            cluster.reset_points();
        }
        for point in POINT_SET {
            for i in 0..NUMBER_OF_CLUSTERS {
                checks[i] = calc_dist::calc_dist(&point, &clusters[i].centroid);
            }
            let min_pos: usize = find_min::find_min(&checks);
            println!("Assigning point {:?} to cluster {}", point, min_pos);
            clusters[min_pos].points.push(point);
        }

        for cluster in &mut clusters {
            totals = [0.0; DIMENSION_OF_POINTS];
            for point in &cluster.points {
                for j in 0..DIMENSION_OF_POINTS {
                    totals[j] += point[j];
                }
            }

            for j in 0..DIMENSION_OF_POINTS {
                cluster.centroid[j] = totals[j] / cluster.points.len() as f64;
            }
        }

        let mut equality = 0;
        for cluster in &mut clusters {
            println!("New Centroid: {:?}", cluster.centroid);
            println!("Old Centroid: {:?}", cluster.old_centroid);
            if cluster.centroid == cluster.old_centroid {
                equality += 1;
            }
            cluster.old_centroid = cluster.centroid.clone();
        }

        if equality == NUMBER_OF_CLUSTERS {
            break
        }
    }

    println!("\nFinal Clusters:\n");
    let mut i = 0;
    let mut total_cluster_variance: f64 = 0.0;
    for cluster in &mut clusters {
        println!("Cluster {}: {:?}", i, cluster.points);
        println!("Centroid: {:?}", cluster.centroid);
        cluster.within_cluster_variance =  within_cluster_variance::within_cluster_variance(&cluster.points, &cluster.centroid);
        total_cluster_variance += cluster.within_cluster_variance;
        println!("Within Cluster Variance: {}\n", cluster.within_cluster_variance);
        i += 1;
    }
    println!("Total Within Cluster Variance: {}", total_cluster_variance);
    println!("\nFinal Centroids:\n");
}
