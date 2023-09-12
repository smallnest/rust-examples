// Idiom #40 Graph with adjacency lists
// Declare a Graph data structure in which each Vertex has a collection of its neighbouring vertices.


use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
struct Vertex {
    name: String,
    neighbours: Vec<Vertex>,
}

#[derive(Default,Debug)]
struct Graph {
    vertices: HashMap<String, Vertex>,
}

fn main() {
    let mut g = Graph {
        vertices: HashMap::new(),
    };
    g.vertices.insert("a".to_string(), Vertex {
        name: "a".to_string(),
        neighbours: vec![
            Vertex {
                name: "b".to_string(),
                neighbours: vec![],
            },
            Vertex {
                name: "c".to_string(),
                neighbours: vec![],
            },
        ],
    });

    println!("{:?}", g)
}