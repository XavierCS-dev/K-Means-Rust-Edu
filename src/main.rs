mod calc_dist;
mod find_min;
mod within_cluster_variance;

// Initialisation constants
pub const NUMBER_OF_CLUSTERS: usize = 2;
pub const NUMBER_OF_POINTS: usize = 8;
pub const DIMENSION_OF_POINTS: usize = 2;

fn main () {

    // Initial cluster points to be put here
    let mut old_points: [[f64;DIMENSION_OF_POINTS]; NUMBER_OF_CLUSTERS] = [[1.0,2.0], [1.0, 4.0]];
    let mut new_points: [[f64;DIMENSION_OF_POINTS]; NUMBER_OF_CLUSTERS] = [[1.0,2.0], [1.0, 4.0]];

    // Points to be classified
    let point_set:[[f64; DIMENSION_OF_POINTS]; NUMBER_OF_POINTS] = [[3.0,1.0], [5.0,1.0], [4.0,2.0], [5.0,2.0], [2.0,5.0], [7.0,4.0], [1.0,0.0], [8.0,0.0]];

    // Cluster sets
    // May be problematic when NUMBER_OF_CLUSTERS > 32
    let mut clusters: [Vec<[f64; DIMENSION_OF_POINTS]>; NUMBER_OF_CLUSTERS] = Default::default();
    let mut passes = 0;
    // for storing the distances
    let mut checks: [f64; NUMBER_OF_CLUSTERS] = [0.0; NUMBER_OF_CLUSTERS];
    // for calculating the new mean
    let mut totals: [f64; DIMENSION_OF_POINTS];

    loop {
        passes += 1;
        println!("\npass: {}\n", passes);
        clusters = Default::default();
        for point in point_set {
            for i in 0..NUMBER_OF_CLUSTERS {
                checks[i] = calc_dist::calc_dist(&point, &new_points[i]);
            }
            let min_pos: usize = find_min::find_min(&checks);
            println!("Assigning point {:?} to cluster {}", point, min_pos);
            clusters[min_pos].push(point);
        }

        let mut i = 0;
        for cluster in &clusters {
            totals = [0.0; DIMENSION_OF_POINTS];
            for point in cluster {
                for j in 0..DIMENSION_OF_POINTS {
                    totals[j] += point[j];
                }
            }

            for j in 0..DIMENSION_OF_POINTS {
                new_points[i][j] = totals[j] / cluster.len() as f64;
            }
            i += 1
        }

        println!("New Centroids: {:?}", new_points);
        println!("Old Centroids: {:?}", old_points);

        if old_points == new_points {
            break
        } else {
            old_points = new_points.clone();
        }
    }

    println!("\nFinal Clusters:\n");
    let mut i = 0;
    let mut total_cluster_variance: f64 = 0.0;
    for cluster in clusters {
        println!("Cluster {}: {:?}", i, cluster);
        let sub_cluster_variance =  within_cluster_variance::within_cluster_variance(&cluster, &new_points[i]);
        total_cluster_variance += sub_cluster_variance;
        println!("Within Cluster Variance: {}\n", sub_cluster_variance);
        i += 1;
    }
    println!("Total Within Cluster Variance: {}", total_cluster_variance);
    println!("\nFinal Centroids:\n");
    i = 0;
    for centroid in new_points {
        println!("Centroid {}: {:?}", i, centroid);
        i += 1;
    }

}
