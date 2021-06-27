// Copyright 2012, University of Freiburg,
// Chair of Algorithms and Data Structures.
// Author: Hannah Bast <bast@informatik.uni-freiburg.de>.

// Disclaimer: this is a *language-unspecific* declaration. Its purpose is to
// provide suggestions on how to design / organize your code. It is up to you
// whether you follow the given advice or do it in some other way.


use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

pub mod osm;

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

// An arc, as used in the adjacency lists below. Note all arcs from a single adjacency
// list are adjacent to the same node, there it suffices to store only the id of
// the node on the other side, the so-called head node of the arc. Arc costs are
// travel times and counted in seconds, that way we can use an integer to store
// them and have no issues with rounding.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Arc {
    // The id of the head node.
    pub head_node_id: usize,
    pub idx: usize,

    // The cost of the arc = travel time in seconds (see class comment above).
    pub cost: usize,
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

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Arc {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Arc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
                &self.adjacent_arcs[*idx_u as usize].push(Arc {head_node_id: v, idx: *idx_v as usize, cost});
                &self.adjacent_arcs[*idx_v as usize].push(Arc {head_node_id: u, idx: *idx_u as usize, cost});
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
                &self.adjacent_arcs[*idx_u as usize].push(Arc {head_node_id: v, idx: *idx_v as usize, cost});
                &self.adjacent_arcs[*idx_v as usize].push(Arc {head_node_id: u, idx: *idx_u as usize, cost});
            }
        }
    }

    pub fn add_one_way_edge(&mut self, tail: usize, head: usize, cost: usize) {
        match (self.node_id_to_index.get(&tail), self.node_id_to_index.get(&head)) {
            (Some(idx_u), Some(idx_v)) => {
                &self.adjacent_arcs[*idx_u as usize].push(Arc {head_node_id: head, idx: *idx_v as usize, cost});
            },
            _ => { /*println!("Warning nodes not found: tail: {}/{:?}, head: {}/{:?}", tail,self.node_id_to_index.get(&tail), head,  self.node_id_to_index.get(&head)); */}
        }
    }

    pub fn reduce_to_largest_connected_component(&mut self) {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut largest_connected_nodes: Option<HashSet<usize>> = None;
        let mut largest_number_of_connected_nodes = 0;
        //println!("Nodes.len(): {}", self.nodes.len());
        for i in 0..self.nodes.len() {
            if visited.contains(&i) { continue };
            visited.insert(i);

            if self.adjacent_arcs[i].len() == 0 { continue; }

            match self.compute_shortest_path(self.nodes[i].osm_id, None) {
                (_, connected_nodes, _) => { 
                    if  connected_nodes.len() > largest_number_of_connected_nodes { 
                        largest_number_of_connected_nodes = connected_nodes.len(); 
                        largest_connected_nodes = Some(connected_nodes.clone());
                    }
                    visited.extend(connected_nodes);
                },
            }
            //break;
        }

        if let Some(largest_connected_nodes) = largest_connected_nodes {
            //println!("Largest connected nodes: {:?}", largest_connected_nodes);
            let mut rn =  RoadNetwork::new();
            for &idx in &largest_connected_nodes {
                rn.add_node(self.nodes[idx as usize].clone());
            }
            //println!("Node id to index: {:?}", rn.node_id_to_index);
            for &idx in &largest_connected_nodes {
                for arc in &self.adjacent_arcs[idx as usize] {
                    if largest_connected_nodes.contains(&arc.idx) {
                        rn.add_one_way_edge(self.nodes[idx].osm_id, arc.head_node_id, arc.cost);
                    }
                }
            };
            println!("Largest number of connected nodes: {:?}", rn.nodes.len());
            self.nodes = rn.nodes;
            self.adjacent_arcs = rn.adjacent_arcs;
            self.node_id_to_index = rn.node_id_to_index;
        }
    }

    // Compute the shortest paths from the given source to the given target node.
    // Returns the cost of the shortest path.
    // NOTE: If called with target node -1, Dijkstra is run until all nodes
    // reachable from the source are settled.
    pub fn compute_shortest_path(&self, source_node_id: usize, target_node_id: Option<usize>) -> (Option<usize>, HashSet<usize>, Option<HashMap<usize, usize>>) {

        let mut visited: HashSet<usize> = HashSet::new();

        let mut distance = vec![usize::MAX; self.nodes.len() as usize];

        let mut previous_node= HashMap::new();

        let mut priority_queue = BinaryHeap::new();

        // Set initial distance to 0 for the source node
        if let Some(&idx) = self.node_id_to_index.get(&source_node_id)  {
            distance[idx as usize] = 0;
            let current_node = Arc {head_node_id: source_node_id, idx: idx as usize, cost: 0};
            priority_queue.push(current_node);
        }


        while let Some(Arc { head_node_id, idx, cost }) = priority_queue.pop() {

           // println!("Processing: {}, idx: {}, cost: {}", head_node_id, idx, cost);
            visited.insert(idx);

            if cost > distance[idx] { continue; }

            distance[idx] = cost;

            if Some(head_node_id) == target_node_id {
                /*
                let mut idx = idx;
                loop {
                    println!("End node/Previous Node {}/{}", self.nodes[idx].osm_id, self.nodes[previous_node[idx]].osm_id);
                    if self.nodes[idx].osm_id == self.nodes[previous_node[idx]].osm_id { break; }
                    if source_node_id as usize == previous_node[idx as usize] { break; }
                    if source_node_id as usize == idx as usize { break; }
                    idx = previous_node[idx as usize];
                }*/
                return (Some(cost), visited, Some(previous_node));
            }
            if cost == usize::MAX {
                return (None, visited, None);
            }
            for &arc in self.adjacent_arcs[idx].iter() {
                if visited.contains(&arc.idx) { continue; }
                //println!("  Neighbor: {:?}: ", arc);
                let next = Arc {
                    head_node_id: arc.head_node_id,
                    idx: arc.idx,
                    cost: arc.cost + cost,
                };
                //println!(" Next.cost: {}, distance[next.idx]: {}", next.cost, distance[next.idx]);
                if next.cost < distance[next.idx] {
                    distance[next.idx] = next.cost;
                    priority_queue.push(next);
                    if target_node_id.is_some() {
                        previous_node.insert(next.idx as usize, idx);
                    }
                }
            }
            //println!("priority_queue: {:?}", priority_queue);
        }

        //println!("Previous node len: {:?}", previous_node.len());
        (None, visited, None)
    }
}
