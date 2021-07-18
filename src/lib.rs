// Author: Vinh-An Trinh
// Copyright 2021

// My implementation of Lecture 1 class given by Prof. Dr. Hannah Bast <bast@informatik.uni-freiburg.de>
// Class wiki: https://ad-wiki.informatik.uni-freiburg.de/teaching/EfficientRoutePlanningSS2012

use std::collections::HashMap;
use std::collections::HashSet;

pub mod osm;
pub mod dijkstra;
pub mod astar_landmark_triangle_inequality;
pub mod arc_flags;

// A node with its OSM id and its latitude / longitude. This is useful for
// building the graph from an OSM file (we first read the nodes there, and later
// have to compute arc costs from the coordinates of these nodes). It is also
// useful for debugging.
#[derive(Copy, Clone, Debug)]
pub struct Node {
    // The OSM id of the node.
    pub osm_id: usize,

    // The latitude and longitude (in radian).  Radian= Degrees * PI / 180
    pub latitude: f64,
    pub longitude: f64,
}

impl Node {
    pub fn cost(&self, v: &Node, speed: usize) -> usize {
        //println!("Node 1: {:?}", self);
        //println!("Node 2: {:?}", v);
        // Quick distance from this node to Arc's node 
        const R: f64 = 6371.0 * 1000.0;
        let x = (v.longitude - self.longitude) * (0.5*(v.latitude + self.latitude)).cos();
        let y = v.latitude - self.latitude;
        ((R * (x*x + y*y).sqrt()) / (speed*5/18) as f64).round() as usize
    }
}

// An arc, as used in the adjacency lists below. Note all arcs from a single adjacency
// list are adjacent to the same node, there it suffices to store only the id of
// the node on the other side, the so-called head node of the arc. Arc costs are
// travel times and counted in seconds, that way we can use an integer to store
// them and have no issues with rounding.
#[derive(Copy, Clone, Debug)]
pub struct Arc {
    // The id of the head node.
    pub head_node_id: usize,
    pub idx: usize,

    // The cost of the arc = travel time in seconds (see class comment above).
    pub cost: usize,
    pub speed: usize,

    pub arc_flag: bool,
}

impl Arc {
    pub fn new(head_node_id: usize, idx: usize, cost: usize, speed: usize) -> Arc {
        Arc { head_node_id, idx, cost, speed, arc_flag: false }
    }
}
// A road network modelled as an undirected graph. We will use "arc" and "edge",
// where "arc" is directed and "edge" is undirected. From the outside, we only
// add "edges", but internally each edge is stored as a pair of "arcs" (with the
// same pair of adjacent nodes but opposite directions).
#[derive(Debug)]
pub struct RoadNetwork {
    // PRIVATE members.

    // The number of nodes in the graph.
    // vtrinh: no need length is returned Vec.len()
    // num_nodes: i32,

    // The number of (undirected) edges in the graph.
    // vtrinh: no need length is returned in adjacent_arcs.len()/2
    // num_edges: i32,

    // The adjacency lists. Note that each edge {u,v} is stored as two arcs: (u,v)
    // and (v,u). The total number of entries in these arrays is therefore exactly twice
    // the number of edges in the graph.
    //adjacent_arcs: Vec<Vec<Arc>>,

    // vtrinh: Wouldn't a Map make more sense?
    pub adjacent_arcs: Vec<Vec<Arc>>,

    // The nodes of the graph.
    pub nodes: Vec<Node>,

    pub node_id_to_index: HashMap<usize, usize>,
}

impl RoadNetwork {
    // PUBLIC members.
    pub fn get_node(&self, osm_id: &usize) -> Option<&mut Node> {
        if let Some(idx) = self.node_id_to_index.get(osm_id) {
            Some(&self.adjacent_arcs[*idx as usize]);
        }
        None
    }

    pub fn get_node_from_lat_lng(&self, lat: &f64, lng: &f64) -> Option<&Node> {
        for i in 0..self.nodes.len() {
            if self.nodes[i].latitude == *lat && self.nodes[i].longitude == *lng {
                return Some(&self.nodes[i]);
            }
        }
        None
    }

    // Create an empty network (with zero nodes and zero arcs).
    pub fn new() -> RoadNetwork {
        RoadNetwork { /*num_nodes: 0, num_edges: 0, */ adjacent_arcs: vec!(), nodes: vec!(), node_id_to_index: HashMap::new()}
    }

    pub fn add_node(&mut self, node: Node) {
        self.node_id_to_index.entry(node.osm_id).or_insert(self.nodes.len() as usize);
        self.nodes.push(node);
        self.adjacent_arcs.push(vec!());
    }

    // Add an (undirected) edge between the given nodes with the given cost.
    // vtrinh: duplicate allowed for now
    pub fn add_edge_calc_cost_from_speed(&mut self, u: usize, v: usize, speed: usize) {
        if let Some(idx_u) = self.node_id_to_index.get(&u) {
            if let Some(idx_v) = self.node_id_to_index.get(&v) {
                let node1 = &self.nodes[*idx_u as usize];
                let node2 = &self.nodes[*idx_v as usize];
                let cost = node1.cost(node2, speed);
                &self.adjacent_arcs[*idx_u as usize].push(Arc::new(v, *idx_v as usize, cost, speed));
                &self.adjacent_arcs[*idx_v as usize].push(Arc::new(u, *idx_u as usize, cost, speed));
            }
            else {
                println!("Warning node not found: {}", v);
            }
        }
        else {
            println!("Warning node not found: {}", u);
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, cost: usize) {
        if let Some(idx_u) = self.node_id_to_index.get(&u) {
            if let Some(idx_v) = self.node_id_to_index.get(&v) {
                &self.adjacent_arcs[*idx_u as usize].push(Arc::new(v, *idx_v as usize, cost, 0));
                &self.adjacent_arcs[*idx_v as usize].push(Arc::new(u, *idx_u as usize, cost, 0));
            }
        }
    }

    pub fn add_one_way_edge(&mut self, tail: usize, head: usize, cost: usize, speed: usize) {
        match (self.node_id_to_index.get(&tail), self.node_id_to_index.get(&head)) {
            (Some(idx_u), Some(idx_v)) => {
                &self.adjacent_arcs[*idx_u as usize].push(Arc::new(head, *idx_v as usize, cost, speed));
            },
            _ => { /*println!("Warning nodes not found: tail: {}/{:?}, head: {}/{:?}", tail,self.node_id_to_index.get(&tail), head,  self.node_id_to_index.get(&head)); */}
        }
    }

    pub fn reduce_to_largest_connected_component(&mut self) {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut largest_connected_nodes: HashMap<usize, usize> = HashMap::new();
        let mut largest_number_of_connected_nodes = 0;
        //println!("Nodes.len(): {}", self.nodes.len());
        let dijkstra = dijkstra::Dijkstra{ consider_arc_flags: false };

        for i in 0..self.nodes.len() {
            if visited.contains(&i) { continue };
            visited.insert(i);

            if self.adjacent_arcs[i].len() == 0 { continue; }

            match dijkstra.compute_shortest_path(&self.nodes, &mut self.adjacent_arcs, i, None, |_,_| 0) {
                (_, v, previous_nodes, _) => { 
                    if  previous_nodes.len() > largest_number_of_connected_nodes { 
                        largest_number_of_connected_nodes = previous_nodes.len(); 
                        largest_connected_nodes = previous_nodes.clone();
                    }
                    visited.extend(v);
                }
            }
            //break;
        }

        //if let Some(largest_connected_nodes) = largest_connected_nodes {
            //println!("Largest connected nodes: {:?}", largest_connected_nodes);
            let mut rn =  RoadNetwork::new();
            for &idx in largest_connected_nodes.keys() {
                rn.add_node(self.nodes[idx as usize].clone());
            }
            //rn.add_node(self.nodes[idx as usize].clone());
            //println!("Node id to index: {:?}", rn.node_id_to_index);
            for &idx in largest_connected_nodes.keys() {
                for arc in &self.adjacent_arcs[idx as usize] {
                    if largest_connected_nodes.contains_key(&arc.idx) {
                        rn.add_one_way_edge(self.nodes[idx].osm_id, arc.head_node_id, arc.cost, arc.speed);
                    }
                }
            };
            println!("Largest number of connected nodes: {:?}", rn.nodes.len());
            self.nodes = rn.nodes;
            self.adjacent_arcs = rn.adjacent_arcs;
            self.node_id_to_index = rn.node_id_to_index;
        //}
    }

}
