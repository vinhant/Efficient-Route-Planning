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
use rand::prelude::*;
use crate::Arc;
use crate::Node;
use crate::dijkstra;

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

  // Select the given number of landmarks at random.
  pub fn select_landmarks(&mut self, &nodes_len: &usize, &num_landmarks: &usize) -> () {
      self.landmarks = vec![0; num_landmarks];
      self.landmark_distances = vec![vec![0; num_landmarks]; nodes_len];
      let mut rng = thread_rng();
      let distr = rand::distributions::Uniform::new_inclusive(0, nodes_len);
      for l in 0..num_landmarks {
        self.landmarks[l] = rng.sample(distr);
      }
  }

  // Precompute the distances to and from the selected landmarks.
  // NOTE: For our undirected / symmetric graphs, the distances *from* the
  // landmarks are enough, see Array<Array<int>> landmarkDistances below.
  pub fn precompute_landmark_distances(&mut self, nodes: &Vec<Node>, adjacent_arcs: &Vec<Vec<Arc>>) -> () {
      assert_eq!(nodes.len(), self.landmark_distances.len());

      for t in 0..self.landmarks.len() {
          for i in 0..nodes.len() {
              if let (Some(cost), _, _) = dijkstra::compute_shortest_path(nodes, adjacent_arcs, i, Some(t), |_,_| 0) {
                  self.landmark_distances[i][t] = cost;
              }
          }
      }
  }
  
  // Compute the shortest paths from the given source to the given target node,
  // using A* with the landmark heuristic.
  // NOTE: this algorithm only works in point-to-point mode, so the option
  // targetNodeId == -1 does not make sense here.
  pub fn compute_shortest_path(&self, s: usize, t: usize) -> usize {
      0
  }

}
