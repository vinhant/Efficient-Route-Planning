// Copyright 2012, University of Freiburg,
// Chair of Algorithms and Data Structures.
// Author: Hannah Bast <bast@informatik.uni-freiburg.de>.

// Disclaimer: this is a *language-unspecific* declaration. Its purpose is to
// provide suggestions on how to design / organize your code. It is up to you
// whether you follow the given advice or do it in some other way.


// A node with its OSM id and its latitude / longitude. This is useful for
// building the graph from an OSM file (we first read the nodes there, and later
// have to compute arc costs from the coordinates of these nodes). It is also
// useful for debugging.
#[derive(Debug)]
struct Node {
  // The OSM id of the node.
  osmId: i32,

  // The latitude and longitude.
  latitude: f64,
  longitude: f64,
}

// An arc, as used in the adjacency lists below. Note all arcs from a single adjacency
// list are adjacent to the same node, there it suffices to store only the id of
// the node on the other side, the so-called head node of the arc. Arc costs are
// travel times and counted in seconds, that way we can use an integer to store
// them and have no issues with rounding.
#[derive(Debug)]
struct Arc {
  // The id of the head node.
  headNodeId: i32,

  // The cost of the arc = travel time in seconds (see class comment above).
  cost: i32,
}

impl Arc {
    pub fn new(headNodeId: i32, cost: i32) -> Arc {
        Arc { headNodeId, cost }
    }
}

// A road network modelled as an undirected graph. We will use "arc" and "edge",
// where "arc" is directed and "edge" is undirected. From the outside, we only
// add "edges", but internally each edge is stored as a pair of "arcs" (with the
// same pair of adjacent nodes but opposite directions).
#[derive(Debug)]
struct RoadNetwork {
  // PRIVATE members.
  
  // The number of nodes in the graph.
  numNodes: i32,

  // The number of (undirected) edges in the graph.
  numEdges: i32,

  // The adjacency lists. Note that each edge {u,v} is stored as two arcs: (u,v)
  // and (v,u). The total number of entries in these arrays is therefore exactly twice
  // the number of edges in the graph.
  adjacentArcs: Vec<Vec<Arc>>,

  // The nodes of the graph.
  nodes: Vec<Node>,
}

impl RoadNetwork {
// PUBLIC members.
 
  // Create an empty network (with zero nodes and zero arcs).
  pub fn new() -> RoadNetwork {
      RoadNetwork { numNodes: 0, numEdges: 0, adjacentArcs: vec!(vec!()), nodes: vec!()}
  }

  // Add a node with the given OSM id and lat/lng coordinates.
  pub fn addNode(osmId: i32, latitude: f32, longitude: f32) {
  }

  // Add an (undirected) edge between the given nodes with the given cost.
  pub fn addEdge(u: i32, v: i32, cost: i32) {
  }

  // Read graph from given OSM file.
  pub fn readFromOsmFile(fileName: &str) {
  }

}

fn main() {
    let n = Node {   osmId: 0, latitude: 0.0, longitude: 0.0, };
    println!("Node: {:?}", n);

    let a = Arc::new(0, 0);
    println!("Arc: {:?}", a);

    let rn = RoadNetwork::new();
    println!("RoadNetwork: {:?}", rn);
}
