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
  osm_id: i32,

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
  head_node_id: i32,

  // The cost of the arc = travel time in seconds (see class comment above).
  cost: i32,
}

impl Arc {
    pub fn new(head_node_id: i32, cost: i32) -> Arc {
        Arc { head_node_id, cost }
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
  num_nodes: i32,

  // The number of (undirected) edges in the graph.
  num_edges: i32,

  // The adjacency lists. Note that each edge {u,v} is stored as two arcs: (u,v)
  // and (v,u). The total number of entries in these arrays is therefore exactly twice
  // the number of edges in the graph.
  adjacent_arcs: Vec<Vec<Arc>>,

  // The nodes of the graph.
  nodes: Vec<Node>,
}

impl RoadNetwork {
// PUBLIC members.
 
  // Create an empty network (with zero nodes and zero arcs).
  pub fn new() -> RoadNetwork {
      RoadNetwork { num_nodes: 0, num_edges: 0, adjacent_arcs: vec!(vec!()), nodes: vec!()}
  }

  // Add a node with the given OSM id and lat/lng coordinates.
  pub fn add_node(&mut self, osm_id: i32, latitude: f64, longitude: f64) {
      self.nodes.push( Node { osm_id, latitude, longitude });
  }

  // Add an (undirected) edge between the given nodes with the given cost.
  pub fn add_edge(&mut self, u: i32, v: i32, cost: i32) {
  }

  // Read graph from given OSM file.
  pub fn read_from_osm_file(&mut self, filename: &str) {
  }

}

fn main() {
    let n = Node {   osm_id: 0, latitude: 0.0, longitude: 0.0, };
    println!("Node: {:?}", n);

    let a = Arc::new(0, 0);
    println!("Arc: {:?}", a);

    let mut rn = RoadNetwork::new();
    rn.add_node(1, 2.2, 3.3);
    println!("RoadNetwork: {:?}", rn);
}

//
// ASCII graph of the slide
// (1) ----3---- (2)
// \            / |
//  \          /  |
//  1         1   |
//   \       /    3
//    \     /     |
//      (3)       |
//                |
// (4)----5------(5)
//
// Array<Array<Arc>>
// 1 -> [ Arc{nodeId: 2, cost: 3}, Arc{nodeId: 3, cost: 1}]
// 2 -> [ Arc{nodeId: 1, cost: 3}, Arc{nodeId: 3, cost: 1}, Arc{nodeId: 5, cost: 3}]
// 3 -> [ Arc{nodeId: 1, cost: 1}, Arc{nodeId: 2, cost: 1}]
// 4 -> [ Arc{nodeId: 5, cost: 5}]
// 5 -> [ Arc{nodeId: 2, cost: 3}, Arc{nodeId: 4, cost: 5}]

// add_node(100, 0.0, 0.0); add_node(200, 0.0, 0.0); ... add_node(500, 0.0, 0.0);
//   nodes: [ (100, 0.0, 0.0), (200, 0.0, 0.0), (300, 0.0, 0.0), (400, 0.0, 0.0), (500, 0.0, 0.0)]
//   nodes[0] is node 1, nodes[1] is node 2, etc in ASCII graph above
//
// add_edge(u, v, cost)
//   adjacentArcs[u].push(Arc{nodeId: v, cost})
//   adjacentArcs[v].push(Arc{nodeId: u, cost})
//   To add edge from node 1 to 2 with cost 3 in ASCII graph above:
//   add_edge(0, 1, 3);
//   // 
