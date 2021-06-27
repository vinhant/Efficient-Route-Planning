// Copyright 2012, University of Freiburg,
// Chair of Algorithms and Data Structures.
// Author: Hannah Bast <bast@informatik.uni-freiburg.de>.

// Disclaimer: this is a *language-unspecific* declaration. Its purpose is to
// provide suggestions on how to design / organize your code. It is up to you
// whether you follow the given advice or do it in some other way.

use crate::Node;
use crate::RoadNetwork;

use std::str;
use std::borrow::Cow;
use std::error::Error;
use std::io::BufRead;
use std::f64::consts::PI;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::attributes::Attribute;
use quick_xml::Reader;
use quick_xml::events::Event;

//pub mod osm {
    fn add_node_from_event(rn: &mut RoadNetwork, attrs: &mut Attributes)  -> Result<(), Box<dyn Error>> {
        let mut n = Node { osm_id: 0, latitude: 0.0, longitude: 0.0, };
        for attr in attrs {
            //println!("node {:?} attribute key/value: {:?}/{:?}", e.name(), .unwrap(), str::from_utf8(&attr.value).unwrap());
            match attr {
                Ok(Attribute{ key: b"id", value})  => n.osm_id = str::from_utf8(&value)?.parse()?,
                Ok(Attribute{ key: b"lat", value})  => n.latitude = (PI/180.0) * str::from_utf8(&value)?.parse::<f64>()?,
                Ok(Attribute{ key: b"lon", value})  => n.longitude = (PI/180.0) * str::from_utf8(&value)?.parse::<f64>()?,
                _ => continue,
            }
        }
        rn.node_id_to_index.entry(n.osm_id).or_insert(rn.nodes.len() as usize);
        rn.nodes.push(n);
        rn.adjacent_arcs.push(vec!());
        Ok(())
    }

    fn add_edge_from_event<B: BufRead>(rn: &mut RoadNetwork, reader: &mut Reader<B>)  -> Result<(), Box<dyn Error>> {
        let mut buf = Vec::new();

        // Save all the "nd ref" in this vec
        let mut v_nodes:Vec<usize> = vec!();

        let mut speed:usize = 0;
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
                            if let Some(Ok(Attribute {key: b"k", value:Cow::Borrowed( b"highway") })) = iter.next() {
                                if let Some(Ok(Attribute {key: b"v", value: v2})) = iter.next() {
                                    if let Some(c) =  road_type_value(&v2.as_ref()) {
                                        speed = c as usize;
                                        break;
                                    }
                                    else { break; }
                                }
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

        if speed > 0 {
            for i in v_nodes.windows(2) {
                rn.add_edge_calc_cost_from_speed(i[0], i[1], speed);
            }
        }
        Ok(())
    }

    // Read graph from given OSM file.
    pub fn read_from_osm_file(filename: &str) -> Result<RoadNetwork, Box<dyn Error>> {
        let mut rn = RoadNetwork::new();
        let mut reader = Reader::from_file(filename)?;
        reader.trim_text(true);

        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
        loop {
            match reader.read_event(&mut buf)? {
                Event::Empty(e)|Event::Start(e) => match e.name() {
                    b"node" => add_node_from_event(&mut rn, &mut e.attributes())?,
                    b"way" => add_edge_from_event(&mut rn, &mut reader)?,
                    _ => (),
                },
                Event::Eof => break, // exits the loop when reaching end of file
                _ => () // There are several other `Event`s we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }
        Ok(rn)
    }
/*
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
*/
// This seems to be the best type
// impl RoadTypes {
    fn road_type_value(s: &[u8]) -> Option<u32> {
        match s {
            b"motorway" => Some(110), //Some(RoadTypes::Motorway),
            b"trunk" => Some(110), //Some(RoadTypes::Trunk),
            b"primary" => Some(70), //Some(RoadTypes::Primary),
            b"secondary" => Some(60), //Some(RoadTypes::Secondary),
            b"tertiary" => Some(50), //jSome(RoadTypes::Tertiary),
            b"motorway_link" => Some(50), //Some(RoadTypes::MotorwayLink),
            b"trunk_link" => Some(50), //Some(RoadTypes::TrunkLink),
            b"primary_link" => Some(50), //Some(RoadTypes::PrimaryLink),
            b"secondary_link" => Some(50), //Some(RoadTypes::SecondaryLink),
            b"road" => Some(40), //Some(RoadTypes::Road),
            b"unclassified" => Some(40), //Some(RoadTypes::Unclassified),
            b"residential" => Some(30), //jSome(RoadTypes::Residential),
            b"unsurfaced" => Some(30), //jSome(RoadTypes::Unsurfaced),
            b"living_street" => Some(10), //Some(RoadTypes::LivingStreet),
            b"service" => Some(5), //Some(RoadTypes::Service),
            _ => None,
        }
    }
/*
    fn value(&self) -> usize {
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
*/
//}
