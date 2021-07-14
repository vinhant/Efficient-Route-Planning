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


## End of Lecture 3 Thoughts

Thoughts on Rust
- I feel like I'm not getting Option anymore
- After refactoring in smaller files, I'm starting to get modules.  Rust is opinionated about how a project should be laid out.  
- The Rust documentation is really good.  Every function has an example.  Every module has 5 pages of what the module is about.  (I admin I cheated a little bit by using Rust's 
documentation of Dijkstra)
- I'm trying to use `map` everywhere but for loop is sometimes better
- I switched to VS Code.  I gave up on neovim + plugins.  Main reason was lack of debugging but also has a few small issues with rendering code complete and warning/errors.  VS Code worked almost perfectly after installing the plugins.  Only thing not working is running "cargo build" before debugging.

#### A* with straght-line heuristic results
Saarland Results:
Average Cost, visited.len, time per query: 1944, 58461, 38.589222ms

Ba-wu Results:
Average Cost, visited.len, time per query: 6397, 606954, 541.747541ms

#### A* with landmark heuristic results
Saarland
Precompute time: 4.754135s
Average Cost, visited.len, time per query: 1732, 3619, 3.232474ms

Ba-wu Results:
Precompute time: 82.0354238s
Average Cost, visited.len, time per query: 5902, 46619, 50.542873ms

## End of Lecture 4 Thoughts
Thoughts on Rust
- This lecture didn't any new Rust features not used in previous lectures, so not much thoughts


#### Arcs

Saarland Results:
Number of nodes in region: 17923
Precompute time: 17.5840271s
Average Cost, visited.len, time per query: 1399, 11481, 8.284471ms

Ba-wu Results:
Number of nodes in region: 27462
Precompute time: 228.1937336s
Average Cost, visited.len, time per query: 6697, 18220, 23.535766ms
