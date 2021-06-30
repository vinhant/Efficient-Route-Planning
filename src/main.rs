// Copyright 2012, University of Freiburg,
// Chair of Algorithms and Data Structures.
// Author: Hannah Bast <bast@informatik.uni-freiburg.de>.

// Disclaimer: this is a *language-unspecific* declaration. Its purpose is to
// provide suggestions on how to design / organize your code. It is up to you
// whether you follow the given advice or do it in some other way.

use rand::prelude::*;
use std::time::{Duration, Instant};

use efficient_route_planning::osm;
use efficient_route_planning::dijkstra;

fn main() {

    //let mut rn = osm::read_from_osm_file("tests/baden-wuerttemberg.osm").unwrap();
    let mut rn = osm::read_from_osm_file("tests/saarland.osm").unwrap();
    println!("Reducing RoadNetwork");
    rn.reduce_to_largest_connected_component();
    
    println!("RoadNetwork number of nodes: {}", rn.nodes.len());
    let arc_count:usize = rn.adjacent_arcs.iter().map(|e| e.len()).sum();
    println!("RoadNetwork number of arcs: {}", arc_count);

    println!("Node 0: {:?}", rn.nodes[0]);
    println!("Node 0 Arc 0: {:?}, Node: {:?}", rn.adjacent_arcs[0][0], rn.nodes[rn.adjacent_arcs[0][0].idx]);
    let mut total_cost = 0;
    let mut total_visited = 0;
    let mut total_duration = Duration::new(0, 0); 
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(0, rn.nodes.len());
    for _i in 0..100 {
        let (start, stop) = (rng.sample(distr), rng.sample(distr));
        let now = Instant::now();
        //println!("Computing heuristic");
        //println!("H: {:?}", h);
        if let (Some(cost), _, Some(previous_nodes)) = dijkstra::compute_shortest_path(&rn.nodes, &rn.adjacent_arcs, start, Some(stop), |_,_| 0) {
            total_duration = total_duration + now.elapsed();
            total_cost = total_cost + cost;
            total_visited = total_visited + previous_nodes.len();
        }
    }
    println!("Average Cost, visited.len, time per query: {:?}, {}, {:?}", total_cost/100, total_visited/100, total_duration/100);

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
//
#[cfg(test)]
mod test {
    use efficient_route_planning::dijkstra;

    #[test]
    fn test_cost_between_two_nodes() {
        // Empire State Building
        let u= efficient_route_planning::Node { osm_id: 0, latitude: std::f64::consts::PI/180.0 * 40.74853102502252, longitude: std::f64::consts::PI/180.0 * -73.9856614332118};

        // Times Square
        let v= efficient_route_planning::Node { osm_id: 0, latitude: std::f64::consts::PI/180.0 * 40.75453807308639, longitude: std::f64::consts::PI/180.0 * -73.9866689484263};

        // Distance should be around 673 m
        let speed = 1;
        let c = u.cost(&v, 1);
        println!("Distance between Empire State Building and Times Square: {}", c);

        // Fails if distance is greater than 20 meters from 673
        assert!((c as i32 - 673).abs() < 20);
    }

    #[test]
    fn test_graph_from_lecture() {
        let mut rn = efficient_route_planning::RoadNetwork::new();
        rn.add_node(efficient_route_planning::Node {osm_id: 111, latitude: 11.11, longitude: 11.11});
        rn.add_node(efficient_route_planning::Node {osm_id: 222, latitude: 11.11, longitude: 11.11});
        rn.add_node(efficient_route_planning::Node {osm_id: 333, latitude: 11.11, longitude: 11.11});
        rn.add_node(efficient_route_planning::Node {osm_id: 444, latitude: 11.11, longitude: 11.11});
        rn.add_node(efficient_route_planning::Node {osm_id: 555, latitude: 11.11, longitude: 11.11});
        rn.add_node(efficient_route_planning::Node {osm_id: 666, latitude: 11.11, longitude: 11.11});
        rn.add_node(efficient_route_planning::Node {osm_id: 777, latitude: 11.11, longitude: 11.11});

        rn.add_edge(111, 222, 3);
        rn.add_edge(111, 333, 1);
        rn.add_edge(222, 333, 1);
        rn.add_edge(222, 555, 3);
        rn.add_edge(444, 555, 5);
        rn.add_edge(666, 777, 5);
        println!("RoadNetwork: {:?}", rn);

        match dijkstra::compute_shortest_path(&rn, None, 111, Some(444)) {
            (Some(_cost), _, Some(path)) => {
//                println!("Shortest path 111 to 444: {:?}", cost);
                let mut old_idx = rn.node_id_to_index.get(&444).unwrap().clone();
                while let Some(&current_idx) = path.get(&old_idx) { 
                    println!("Node: {:?}", rn.nodes[current_idx as usize]); 
                    old_idx = current_idx;
                }
            },
            _ => (),
        }

/*
        let res = rn.compute_shortest_path(111, Some(666));
        println!("Shortest path 111 to 666: {:?}", res);
        assert_eq!(res.0, None);

        let res = rn.compute_shortest_path(111, None);
        println!("Shortest path 111 to None: {:?}", res);
        assert_eq!(res.0, None);
*/
        rn.reduce_to_largest_connected_component();
        println!("RoadNetwork: {:?}", rn);
    }

}
