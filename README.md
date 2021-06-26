# Efficient-Route-Planning
My implementation of the class given by Prof. Dr. Hannah Bast

Class wiki: https://ad-wiki.informatik.uni-freiburg.de/teaching/EfficientRoutePlanningSS2012

## End of Lecture 1 Thoughts

Thoughts on Rust after Lecture 1:
- "match" and Result<T, E> are cool
- Not sure on enum Option { Some, None }  
- Rust various smart pointers are hard to keep track of
- Still don't know what I'm really doing, had to ask on Rust Discord server a few things

Thoughts on Lecture 1:
- Why keep adjacent arcs in a Vec<Vec<Arc>> and nodes in Vec<Node>?  I think HashMap<Node, Vec<Arc>> does both?  Maybe we'll find out later
- Parsing a 3GB in reasonable time is part of the challenge

My OSM graph results
Number of nodes (Saarland/Bawu): 1,119,289 / 14,593,458
Number of arcs (Saarland/Bawu): 227,826 / 2,642,949
Time to read + construct:  2.8s / 23.2s
Processor / RAM: Intel i5-4590S at 3.00GHz / 6 GB DDR3
Language: Rust

## End of Lecture 2 Thoughts

Thoughts on Rust
- I'm starting to "get" Option.  You can use as a reult (myoption.unwrap()), you can use it as a collection (myoption.map()).  Also forces you to handle null values with "if let Some(myoption)..." or "match Some(myoption)..."
- Would be nice to have an IDE ready to go.  Visual Studio for C++, Eclipse for Java, etc.  My current setup for Rust is Neovim + LSP + rust-analyzer.  User experience is ok.  
-I'm starting to understand borrowing rules and Rust in general (in regards to programming Lecture 1 and 2).  Have not had to ask Rust Discord

Saaarland results:
RoadNetwork number of nodes: 213567
RoadNetwork number of arcs: 451012
Average Cost, visited.len, time per query: 1798s, 111599, 58.676939ms

Ba-wu results:
RoadNetwork number of nodes: 2458230
RoadNetwork number of arcs: 5226676
Average Cost, visited.len, time per query: 5629, 1186284, 879.037662ms

Processor / RAM: Intel i5-4590S at 3.00GHz / 6 GB DDR3
Language: Rust 1.53
