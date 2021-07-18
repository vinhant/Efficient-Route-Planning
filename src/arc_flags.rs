// Author: Vinh-An Trinh
// Copyright 2021

// My implementation of Lecture 4 class given by Prof. Dr. Hannah Bast <bast@informatik.uni-freiburg.de>
// Class wiki: https://ad-wiki.informatik.uni-freiburg.de/teaching/EfficientRoutePlanningSS2012

// Implementation of Arc Flags algorithm, for a SINGLE REGION ONLY. That's
// enough to get the idea. Implementing it for a full division into region would
// be a lot of additional work, with relatively little additional insight.


use std::f64::consts::PI;
use std::collections::HashSet;
use std::time::{Duration, Instant};
use crate::Arc;
use crate::Node;
use crate::dijkstra::Dijkstra;

pub struct ArcFlagsAlgorithm {
}  

impl ArcFlagsAlgorithm {
    pub fn is_node_in_region(node: &Node, lat_min: f64, lat_max: f64, lng_min: f64, lng_max: f64) -> bool {
        node.latitude < lat_max && node.latitude > lat_min && node.longitude < lng_max && node.longitude > lng_min
    }

    // Precompute arc flags for the given (single, see above) region.
    // NOTE: the arg flags are stored *not* in this object, but as a bit in each
    // Arc of the graph (to which this object has a reference).
    pub fn precompute_arc_flags(&self, nodes: &Vec<Node>, adjacent_arcs: &mut Vec<Vec<Arc>>, lat_min: f64, lat_max: f64, lng_min: f64, lng_max: f64 ) -> Vec<usize> {
        let mut total_duration = Duration::new(0, 0); 
        //let mut visited: HashSet<usize> = HashSet::new();

        let now = Instant::now();
        let dijkstra = Dijkstra { consider_arc_flags: false};
        let r_lat_min=(PI/180.0)*lat_min;
        let r_lat_max=(PI/180.0)*lat_max;
        let r_lng_min=(PI/180.0)*lng_min;
        let r_lng_max=(PI/180.0)*lng_max;

        let mut inside_region: Vec<usize> = vec![];
        for u in 0..nodes.len() {
            if ArcFlagsAlgorithm::is_node_in_region(&nodes[u], r_lat_min, r_lat_max, r_lng_min, r_lng_max) {
                inside_region.push(u);
            }
        }

        println!("Number of nodes in region: {}", inside_region.len());

        for i in 0..inside_region.len() {
            let u = inside_region[i];
            //if visited.contains(&u) { continue; }

            //visited.insert(u);
            let mut boundary_node_processed = false;
            for j in 0..adjacent_arcs[u].len() {
                let v = adjacent_arcs[u][j].idx;
                //if visited.contains(&v) { continue; }

                // visited.insert(v);
                // source and destination are both inside the region
                if inside_region.contains(&v) {
                    adjacent_arcs[u][j].arc_flag=true;
                    // looping over inside_Region will catch the reverse way
                    //adjacent_arcs[v][u].arc_flag=true;       
                    continue;
                }

                // Only compute this once
                if !boundary_node_processed {
                    boundary_node_processed = true;
                    let (_, _, previous_node, _) = dijkstra.compute_shortest_path(nodes, adjacent_arcs, u, None, |_,_| 0);
                    //println!("i/Visited.len/previous_node.len: {}/{}/{}", i, visited.len(), previous_node.len());
                    // Need to set the arc_flag on the reverse way
                    for (head, tail) in previous_node {
                        for arc in &mut adjacent_arcs[head] {
                            if arc.idx == tail { 
                                arc.arc_flag = true; 
                                break;
                            }
                        }
                        /*
                        for arc in &mut adjacent_arcs[v] {
                            if arc.idx == u { 
                                arc.arc_flag = true; 
                            }
                        }
                        */
                    }
                }
            }
        }
        total_duration = total_duration + now.elapsed();
        println!("Precompute time: {:?}",  total_duration);
        inside_region
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
        println!("Cost: {:?}", cost);
        (cost, visited)
    }
}