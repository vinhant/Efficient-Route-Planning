use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::Arc;
use crate::Node;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    idx: usize,
    cost: usize,
    f_score: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.f_score.cmp(&self.f_score)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Dijkstra {
    pub consider_arc_flags: bool,
}

impl Dijkstra {
    // Compute the shortest paths from the given source to the given target node.
    // Returns the cost of the shortest path.
    // NOTE: If called with target node -1, Dijkstra is run until all nodes
    // reachable from the source are settled.
    //pub fn compute_shortest_path(rn: &RoadNetwork, h: Option<&Vec<usize>>, source_node_id: usize, target_node_id: Option<usize>) -> (Option<usize>, HashSet<usize>, Option<HashMap<usize, usize>>) {
    pub fn compute_shortest_path<F>(&self, nodes: &Vec<Node>, arcs: &mut Vec<Vec<Arc>>, s: usize, t: Option<usize>, h: F) -> (Option<usize>, HashSet<usize>, HashMap<usize, usize>, Vec<usize>) where 
    F: Fn(&usize, &usize) -> usize
    {

        assert!(s < nodes.len()-1);
        if let Some(t) = t {
            assert!(t < nodes.len()-1);
        }


        let mut visited: HashSet<usize> = HashSet::new();

        // g_score[n] is the cost from start to n
        let mut g_score = vec![usize::MAX; nodes.len() as usize];

        // f_score[n] is the cost from start to n + a cost estimate of n to target
        //let mut f_score = vec![usize::MAX; nodes.len() as usize];

        let mut previous_node= HashMap::new();

        let mut priority_queue = BinaryHeap::new();

        // Set initial g_score to 0 for the source node
        g_score[s] = 0;
        let mut current_node = State {idx: s, cost: 0, f_score: 0};
        if let Some(t) = t {
            //current_node.f_score = nodes[s].cost(&nodes[t], crate::osm::MAX_SPEED);
            current_node.f_score = h(&s, &t);
        }
        priority_queue.push(current_node);


        while let Some(State {idx, cost, f_score: _}) = priority_queue.pop() {

            // println!("Processing: {}, idx: {}, cost: {}", head_node_id, idx, cost);
            visited.insert(idx);

            if Some(idx) == t {
                return (Some(g_score[idx]), visited, previous_node, g_score);
            }

            for arc in arcs[idx].iter_mut() {
                if visited.contains(&arc.idx) { continue; }
                if self.consider_arc_flags == true && arc.arc_flag == false { continue; }

                if arc.cost + cost < g_score[arc.idx] {
                    g_score[arc.idx] = arc.cost + cost;
                    let mut h_value = 0;
                    if let Some(t) = t  {
                        h_value = h(&arc.idx, &t);
                    }
                    priority_queue.push(State{idx: arc.idx, cost: arc.cost+cost, f_score: g_score[arc.idx] + h_value});
                    previous_node.insert(arc.idx as usize, idx);
                    arc.arc_flag=true;
                }
            }
            //println!("priority_queue: {:?}", priority_queue);
        }

        //println!("Previous node len: {:?}", previous_node.len());
        //println!("Target not reached: {}/{:?}", source_node_id, target_node_id);
        (None, visited, previous_node, g_score)
    }
  
}
