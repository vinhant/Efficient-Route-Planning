use regex::Regex;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use efficient_route_planning::osm;
use efficient_route_planning::RoadNetwork;
use efficient_route_planning::astar_landmark_triangle_inequality::LandmarkAlgorithm;

fn main() -> Result<(), Box<std::io::Error>> {

    let mut rn = osm::read_from_osm_file("tests/baden-wuerttemberg.osm").unwrap();
    //let mut rn = osm::read_from_osm_file("tests/saarland.osm").unwrap();
    println!("Reducing RoadNetwork");
    rn.reduce_to_largest_connected_component();

    let listener = TcpListener::bind("127.0.0.1:8888")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(&rn, stream);
    }
    Ok(())
}

fn handle_connection(rn: &RoadNetwork, mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let mut resp = String::new();
    let req = String::from_utf8_lossy(&buffer[..]);
    if let Some(get) = req.lines().next() {
        println!("Request: {}", get);

        // Extract four floats from "GET /?a,b,c,d HTTP/1.1"
        let re = Regex::new(r"(?x)
        (?P<lat1>[+-]?([0-9]*[.])?[0-9]+)
        ,
        (?P<lng1>[+-]?([0-9]*[.])?[0-9]+)
        ,
        (?P<lat2>[+-]?([0-9]*[.])?[0-9]+)
        ,
        (?P<lng2>[+-]?([0-9]*[.])?[0-9]+)
        ").unwrap();

        let caps = re.captures(get).unwrap();
        println!("Caps: {:?}", caps);
        let s = rn.get_node_from_lat_lng(&caps["lat1"].parse().unwrap(), &caps["lng1"].parse().unwrap());
        let t = rn.get_node_from_lat_lng(&caps["lat2"].parse().unwrap(), &caps["lng2"].parse().unwrap());
        if let (Some(s), Some(t)) = (s, t) {
            println!("s/t: {:?}/{:?}", s, t);
        }

        // Send JSONP results string back to client.
        resp = format!("redrawLineServerCallback({{ \
            path: [{}, {}, {}, {}] \
        }})", &caps["lat1"], &caps["lng1"], &caps["lat2"], &caps["lng2"]);
    }


    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        "HTTP/1.1 200 OK",
        resp.len(),
        resp
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();}