use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::RoadNetwork;
use crate::Arc;

// Compute the shortest paths from the given source to the given target node.
// Returns the cost of the shortest path.
// NOTE: If called with target node -1, Dijkstra is run until all nodes
// reachable from the source are settled.
pub fn compute_shortest_path(rn: &RoadNetwork, source_node_id: usize, target_node_id: Option<usize>) -> (Option<usize>, HashSet<usize>, Option<HashMap<usize, usize>>) {

    let mut visited: HashSet<usize> = HashSet::new();

    let mut distance = vec![usize::MAX; rn.nodes.len() as usize];

    let mut previous_node= HashMap::new();

    let mut priority_queue = BinaryHeap::new();

    // Set initial distance to 0 for the source node
    if let Some(&idx) = rn.node_id_to_index.get(&source_node_id)  {
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
            return (Some(cost), visited, Some(previous_node));
        }
        if cost == usize::MAX {
            return (None, visited, None);
        }
        for &arc in rn.adjacent_arcs[idx].iter() {
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

