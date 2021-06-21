// Copyright 2012, University of Freiburg,
// Chair of Algorithms and Data Structures.
// Author: Hannah Bast <bast@informatik.uni-freiburg.de>.

// Disclaimer: this is a *language-unspecific* declaration. Its purpose is to
// provide suggestions on how to design / organize your code. It is up to you
// whether you follow the given advice or do it in some other way.

//use phf::phf_map;
use std::str;
use std::collections::HashMap;
use std::error::Error;
use std::io::BufRead;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::attributes::Attribute;
use quick_xml::Reader;
use quick_xml::events::Event;

// A node with its OSM id and its latitude / longitude. This is useful for
// building the graph from an OSM file (we first read the nodes there, and later
// have to compute arc costs from the coordinates of these nodes). It is also
// useful for debugging.
#[derive(Debug)]
struct Node {
    // The OSM id of the node.
    osm_id: usize,

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
    head_node_id: usize,

    // The cost of the arc = travel time in seconds (see class comment above).
    cost: i32,
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
    adjacent_arcs: Vec<Vec<Arc>>,

    // The nodes of the graph.
    nodes: Vec<Node>,

    node_id_to_index: HashMap<usize, usize>,
}

impl RoadNetwork {
    // PUBLIC members.

    // Create an empty network (with zero nodes and zero arcs).
    pub fn new() -> RoadNetwork {
        RoadNetwork { /*num_nodes: 0, num_edges: 0, */ adjacent_arcs: vec!(), nodes: vec!(), node_id_to_index: HashMap::new()}
    }

    pub fn add_node_from_event(&mut self, attrs: &mut Attributes)  -> Result<(), Box<dyn Error>> {
        let mut n = Node { osm_id: 0, latitude: 0.0, longitude: 0.0 };
        for attr in attrs {
            //println!("node {:?} attribute key/value: {:?}/{:?}", e.name(), .unwrap(), str::from_utf8(&attr.value).unwrap());
            match attr {
                Ok(Attribute{ key: b"id", value})  => n.osm_id = str::from_utf8(&value)?.parse()?,
                Ok(Attribute{ key: b"lat", value})  => n.latitude = str::from_utf8(&value)?.parse()?,
                Ok(Attribute{ key: b"lon", value})  => n.longitude = str::from_utf8(&value)?.parse()?,
                _ => break,
            }
        }
        self.node_id_to_index.entry(n.osm_id).or_insert(self.nodes.len());
        self.nodes.push(n);
        self.adjacent_arcs.push(vec!());
        Ok(())
    }

    // Add an (undirected) edge between the given nodes with the given cost.
    // vtrinh: duplicate allowed for now
    pub fn add_edge(&mut self, u: usize, v: usize, cost: i32) {
        if let Some(idx_u) = self.node_id_to_index.get(&u) {
            if let Some(idx_v) = self.node_id_to_index.get(&v) {
                &self.adjacent_arcs[*idx_u].push(Arc {head_node_id: v, cost });
                &self.adjacent_arcs[*idx_v].push(Arc {head_node_id: u, cost });
            }
        }
    }

    pub fn add_edge_from_event<B: BufRead>(&mut self, reader: &mut Reader<B>)  -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();

        // Save all the "nd ref" in this vec
        let mut v_nodes:Vec<usize> = vec!();

        let mut cost = 0;
        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {

            match reader.read_event(&mut buf)? {
                // Exit when we see </way>
                Event::End(e) if e.name() == b"way" => { break; },

                Event::Empty(e)|Event::Start(e) => {
                    match e.name() {
                        b"nd" =>  { 
                            if let Some(Ok(attr)) = e.attributes().next() {
                                v_nodes.push(str::from_utf8(&attr.value)?.parse()?);
                            }
                        },
                        b"tag" =>  {
                            let mut iter = e.attributes();
                            // Only process tag of type k="highway" and v="road type in enum
                            // RoadTypes"
                            if let Some(Ok(Attribute {key: b"k", value: v})) = iter.next() {
                                if v.as_ref() == b"highway" {
                                    if let Some(Ok(Attribute {key: b"v", value: v2})) = iter.next() {
                                        if let Some(c) =  RoadTypes::from_string(&v2.as_ref()) {
                                            cost = c.value();
                                            break;
                                        }
                                        else { break; }
                                    }
                                }
                                else { break; }
                            }
                        },
                        _ => (),
                    }
                },
                _ => () // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        if cost > 0 {
            for i in v_nodes.windows(2) {
                self.add_edge(i[0], i[1], cost);
            }
        }
        Ok(())
    }

    // Read graph from given OSM file.
    pub fn read_from_osm_file(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut reader = Reader::from_file(filename)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf)? {
                Event::Empty(e)|Event::Start(e) => match e.name() {
                    b"node" => self.add_node_from_event(&mut e.attributes())?,
                    b"way" => self.add_edge_from_event(&mut reader)?,
                    _ => (),
                },
                Event::Eof => break, // exits the loop when reaching end of file
                _ => () // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        Ok(())
    }

}
// vtrinh: Should roaad types be a map or an enum? Let's do both
// 
// Option 1. A static hashmap.  I like that it doesn't a fromString()
// as the key is the road type as is
// Downside value is wrapped in Option
/*
   static ROADTYPES: phf::Map<&'static str, u32> = phf_map! {
   "motorway" => 110,
   "trunk" => 110,
   "primary" => 70,
   "secondary" => 60,
   "tertiary" => 50,
   "motorway_link" => 50,
   "trunk_link" => 50,
   "primary_link" => 50,
   "secondary_link" => 50,
   "road" => 40,
   "unclassified" => 40,
   "residential" => 30,
   "unsurfaced" => 30,
   "living_street" => 10,
   "service" => 5,
   };
   */
// Option 2. Enum with value.  
// I don't like that I need to pattern match in order to get the inside value.
/*
#[derive(Debug)]
enum RoadTypesWithValue {
motorway(u8),
trunk(u8),
primary(u8),
}
*/

// Option 3. Simple Enum.  Implementation will provide fromString(), value(), etc.
// Downside more code
#[derive(Debug)]
enum RoadTypes {
    Motorway,
        Trunk,
        Primary,
        Secondary,
        Tertiary,
        MotorwayLink,
        TrunkLink,
        PrimaryLink,
        SecondaryLink,
        Road,
        Unclassified,
        Residential,
        Unsurfaced,
        LivingStreet,
        Service,
}

// This seems to be the best type
impl RoadTypes {
    fn from_string(s: &[u8]) -> Option<RoadTypes> {
        match s {
            b"motorway" => Some(RoadTypes::Motorway),
            b"trunk" => Some(RoadTypes::Trunk),
            b"primary" => Some(RoadTypes::Primary),
            b"secondary" => Some(RoadTypes::Secondary),
            b"tertiary" => Some(RoadTypes::Tertiary),
            b"motorway_link" => Some(RoadTypes::MotorwayLink),
            b"trunk_link" => Some(RoadTypes::TrunkLink),
            b"primary_link" => Some(RoadTypes::PrimaryLink),
            b"secondary_link" => Some(RoadTypes::SecondaryLink),
            b"road" => Some(RoadTypes::Road),
            b"unclassified" => Some(RoadTypes::Unclassified),
            b"residential" => Some(RoadTypes::Residential),
            b"unsurfaced" => Some(RoadTypes::Unsurfaced),
            b"living_street" => Some(RoadTypes::LivingStreet),
            b"service" => Some(RoadTypes::Service),
            _ => None,
        }
    }
    fn value(&self) -> i32 {
        match *self {
            RoadTypes::Motorway => 110,
            RoadTypes::Trunk => 110,
            RoadTypes::Primary => 70,
            RoadTypes::Secondary => 60,
            RoadTypes::Tertiary => 50,
            RoadTypes::MotorwayLink => 50,
            RoadTypes::TrunkLink => 50,
            RoadTypes::PrimaryLink => 50,
            RoadTypes::SecondaryLink => 50,
            RoadTypes::Road => 40,
            RoadTypes::Unclassified => 40,
            RoadTypes::Residential => 30,
            RoadTypes::Unsurfaced => 30,
            RoadTypes::LivingStreet => 10,
            RoadTypes::Service => 5,
        }
    }
}



fn main() {

    let mut rn = crate::RoadNetwork::new();
    //rn.read_from_osm_file("tests/quick_xml_reader.xml").unwrap();
    //rn.read_from_osm_file("tests/wiki_example_osm.xml").unwrap();
    //jrn.read_from_osm_file("tests/baden-wuerttemberg.osm").unwrap();
    rn.read_from_osm_file("tests/saarland.osm").unwrap();
    println!("RoadNetwork number of nodes: {}", rn.nodes.len());
    let arc_count:usize = rn.adjacent_arcs.iter().map(|e| e.len()).sum();
    println!("RoadNetwork number of arcs: {}", arc_count);
    for i in 0..100 {
        if  rn.adjacent_arcs[i].len() > 5 {
            println!("RoadNetwork node: {:?}, arcs: {:?}", rn.nodes[i], rn.adjacent_arcs[i]);
        }
    }
    //jprintln!("RoadNetwork arcs: {:?}", rn.adjacent_arcs.get(&500863));
    /*
       let m = RoadTypes::Motorway;
       println!("Road type/value: {:?}/{:?}", m, m.value());    

       if let Some(m) = RoadTypes::from_string("living_street") {
       println!("Road type/value: {:?}/{:?}", m, m.value());    
       }
       */


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
    #[test]
    fn test_graph_from_lecture() {
        let mut rn = crate::RoadNetwork::new();
        rn.add_node(111, 11.11, 11.11);
        rn.add_node(222, 22.22, 22.22);
        rn.add_node(333, 33.33, 33.33);
        rn.add_node(444, 44.44, 44.44);
        rn.add_node(555, 55.55, 55.55);

        rn.add_edge(111, 222, 3);
        rn.add_edge(111, 333, 1);
        rn.add_edge(222, 333, 1);
        rn.add_edge(222, 555, 3);
        rn.add_edge(444, 555, 5);
        println!("RoadNetwork: {:?}", rn);
    }

    fn test_xml_from_osm_wiki() {
        let xml = r#""#;
    }

}
