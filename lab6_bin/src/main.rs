use lab6::graph::{Edge, Graph, Vertex};
use lab6::shortest_path;
use lab6::shortest_path::floyd_shortest_path;

fn print_shortest_path<const V: usize>(graph: &Graph<V>, v1: Vertex, v2: Vertex) {
    let message = match floyd_shortest_path(&graph, v1, v2) {
        Ok(path) => {
            format!("Shortest path between {} and {} = {:?}", v1, v2, path)
        }
        Err(shortest_path::Error::NoShortestPath) => {
            "No shortest path".to_string()
        }
        Err(shortest_path::Error::InvalidVertices(vertices)) => {
            format!("Invalid vertices: {:?}", vertices)
        }
    };

    println!("{}", message);
}

fn main() {
    let mut graph = Graph::<10>::default();

    assert!(graph.add_edge(Edge(0, 1, 2)).is_ok());
    assert!(graph.add_edge(Edge(1, 2, 4)).is_ok());
    assert!(graph.add_edge(Edge(2, 3, 1)).is_ok());
    assert!(graph.add_edge(Edge(3, 4, 7)).is_ok());
    assert!(graph.add_edge(Edge(4, 5, 3)).is_ok());
    assert!(graph.add_edge(Edge(5, 6, 1)).is_ok());
    assert!(graph.add_edge(Edge(6, 7, 5)).is_ok());
    assert!(graph.add_edge(Edge(7, 8, 2)).is_ok());
    assert!(graph.add_edge(Edge(8, 9, 6)).is_ok());

    assert!(graph.add_edge(Edge(0, 3, 10)).is_ok());
    assert!(graph.add_edge(Edge(1, 4, 8)).is_ok());
    assert!(graph.add_edge(Edge(2, 5, 2)).is_ok());
    assert!(graph.add_edge(Edge(3, 6, 4)).is_ok());
    assert!(graph.add_edge(Edge(4, 7, 6)).is_ok());
    assert!(graph.add_edge(Edge(5, 8, 3)).is_ok());
    assert!(graph.add_edge(Edge(6, 9, 7)).is_ok());

    let psp = |v1: Vertex, v2: Vertex| {
        print_shortest_path(&graph, v1, v2);
    };

    psp(0, 9);
    psp(1, 8);
    psp(7, 5);
    psp(5, 7);
    psp(5, 5);
}
