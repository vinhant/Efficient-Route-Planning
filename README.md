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


