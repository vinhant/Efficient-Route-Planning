// Copyright 2012, University of Freiburg,
// Chair of Algorithms and Data Structures.
// Author: Hannah Bast <bast@informatik.uni-freiburg.de>.

// Disclaimer: this is a *language-unspecific* declaration. Its purpose is to
// provide suggestions on how to design / organize your code. It is up to you
// whether you follow the given advice or do it in some other way.


// An implementation of A* with the landmarks heuristic, as explained in
// Lecture 3 of the course Efficient Route Planning, SS 2012.
//
// vinh: Andrew Goldberg from MS Research first published this algorithm
// known as "A* Landmark with Triangle Inequality"
use std::collections::HashSet;
use std::time::{Duration, Instant};
use rand::prelude::*;
use crate::Arc;
use crate::Node;
use crate::dijkstra::Dijkstra;

pub struct LandmarkAlgorithm {
  // PUBLIC members.
  // The set of landmarks. Each entry in the array is a node id.
  landmarks: Vec<usize>,

  // Precomputed distances (shorted path costs in seconds) to and from these
  // landmarks. This is one array of size #nodes per landmark.
  // NOTE: since our graphs are undirected (or rather, for each arc u,v we also
  // have an arc v,u with the same cost) we have dist(u, l) = dist(l, u) and it
  // suffices to store one distance array per landmark. For arbitrary directed
  // graphs we would need one array for the distances *to* the landmark and one
  // array for the distances *from* the landmark.
  landmark_distances: Vec<Vec<usize>>,
}

impl LandmarkAlgorithm {
    pub fn new(nodes: &Vec<Node>, adjacent_arcs: &mut Vec<Vec<Arc>>, num_landmarks: usize) -> LandmarkAlgorithm {
        let mut alt = LandmarkAlgorithm{ landmarks: vec![0; num_landmarks], landmark_distances: vec![vec![0; num_landmarks]; nodes.len()] }; 
        alt.select_landmarks(nodes.len(), num_landmarks);
        alt.precompute_landmark_distances(nodes, adjacent_arcs);
        alt
    }

    // Select the given number of landmarks at random.
    pub fn select_landmarks(&mut self, nodes_len: usize, num_landmarks: usize) -> () {
        let mut rng = thread_rng();
        let distr = rand::distributions::Uniform::new_inclusive(0, nodes_len);
        for l in 0..num_landmarks {
            self.landmarks[l] = rng.sample(distr);
        }
    }

    // Precompute the distances to and from the selected landmarks.
    // NOTE: For our undirected / symmetric graphs, the distances *from* the
    // landmarks are enough, see Array<Array<int>> landmarkDistances below.
    pub fn precompute_landmark_distances(&mut self, nodes: &Vec<Node>, adjacent_arcs: &mut Vec<Vec<Arc>>) -> () {
        assert_eq!(nodes.len(), self.landmark_distances.len());

        let mut total_duration = Duration::new(0, 0); 
        let now = Instant::now();

        let dijkstra = Dijkstra { consider_arc_flags: false};
        for t in 0..self.landmarks.len() {
            let (_, _, _, g_score) = dijkstra.compute_shortest_path(nodes, adjacent_arcs, self.landmarks[t], None, |_,_| 0);
            for i in 0..g_score.len() {
                    self.landmark_distances[i][t] = g_score[i];
            }
        }
        total_duration = total_duration + now.elapsed();
        println!("Precompute time: {:?}",  total_duration);
    }

    fn cost(&self, l: usize, u: usize, v: usize) -> usize {
        (self.landmark_distances[u][l] as i32 - self.landmark_distances[v][l] as i32).abs() as usize
    }

    // Compute the shortest paths from the given source to the given target node,
    // using A* with the landmark heuristic.
    // NOTE: this algorithm only works in point-to-point mode, so the option
    // targetNodeId == -1 does not make sense here.
    pub fn compute_shortest_path(&self, nodes: &Vec<Node>, adjacent_arcs: &mut Vec<Vec<Arc>>, s: usize, t: usize) -> (Option<usize>, HashSet<usize>) {

        let dijkstra = Dijkstra { consider_arc_flags: false};
        let (cost, visited, _, _) = dijkstra.compute_shortest_path(
            nodes, 
            adjacent_arcs, 
            s, 
            Some(t), 
            |&u,_| {
                let mut max = 0;
                for i in 0..self.landmarks.len() {
                    let cost = self.cost(i, u, t);
                    if cost > max { max = cost; }
                }
                max
            }
            ) ;
        (cost, visited)
    }

}
