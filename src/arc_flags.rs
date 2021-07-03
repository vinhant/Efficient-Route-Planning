// Copyright 2012, University of Freiburg,
// Chair of Algorithms and Data Structures.
// Author: Hannah Bast <bast@informatik.uni-freiburg.de>.

use std::f64::consts::PI;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use crate::Arc;
use crate::Node;
use crate::dijkstra::Dijkstra;

// Implementation of Arc Flags algorithm, for a SINGLE REGION ONLY. That's
// enough to get the idea. Implementing it for a full division into region would
// be a lot of additional work, with relatively little additional insight.
pub struct ArcFlagsAlgorithm {
}  

impl ArcFlagsAlgorithm {
    fn is_node_in_region(node: &Node, lat_min: f64, lat_max: f64, lng_min: f64, lng_max: f64) -> bool {
        node.latitude < lat_max && node.latitude > lat_min && node.longitude < lng_max && node.longitude > lng_min
    }

    // Precompute arc flags for the given (single, see above) region.
    // NOTE: the arg flags are stored *not* in this object, but as a bit in each
    // Arc of the graph (to which this object has a reference).
    pub fn precompute_arc_flags(&self, nodes: &Vec<Node>, adjacent_arcs: &mut Vec<Vec<Arc>>, lat_min: f64, lat_max: f64, lng_min: f64, lng_max: f64 ) {
        let mut total_duration = Duration::new(0, 0); 
        let now = Instant::now();
        let dijkstra = Dijkstra { consider_arc_flags: false};
        let r_lat_min=(PI/180.0)*lat_min;
        let r_lat_max=(PI/180.0)*lat_max;
        let r_lng_min=(PI/180.0)*lng_min;
        let r_lng_max=(PI/180.0)*lng_max;
        for u in 0..nodes.len() {
            let b1 = ArcFlagsAlgorithm::is_node_in_region(&nodes[u], r_lat_min, r_lat_max, r_lng_min, r_lng_max);
            if b1 {
                for v in 0..adjacent_arcs[u].len() {
                    let b2 = ArcFlagsAlgorithm::is_node_in_region(&nodes[adjacent_arcs[u][v].idx], lat_min, lat_max, lng_min, lng_max);
                    if b2 == false {
                        dijkstra.compute_shortest_path(nodes, adjacent_arcs, u, None, |_,_| 0);
                    }
                    else {
                        adjacent_arcs[u][v].arc_flag=true;
                    }
                }
            }
        }
        total_duration = total_duration + now.elapsed();
        println!("Precompute time: {:?}",  total_duration);
    }
  
    // Compute the shortest paths from the given source to the given target node,
    // using the precomputed arc flags.
    // PRECONDITION: the target node must be *within* the precomputed region.
    pub fn compute_shortest_path(&self, nodes: &Vec<Node>, adjacent_arcs: &mut Vec<Vec<Arc>>, s: usize, t: usize) -> (Option<usize>, HashSet<usize>) {
        let dijkstra = Dijkstra { consider_arc_flags: true };
        let (cost, visited, _, _) = dijkstra.compute_shortest_path(
            nodes, 
            adjacent_arcs, 
            s, 
            Some(t), 
            |_,_| 0) ;
        (cost, visited)
    }
}