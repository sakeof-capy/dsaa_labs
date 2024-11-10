pub mod graph;

#[cfg(test)]
mod tests {
    use crate::graph::{Edge, Graph};

    #[test]
    fn test_graph() {
        let mut graph = Graph::<10>::default();
        assert!(graph.add_edge(Edge(1, 2)).is_ok());
        assert!(graph.add_edge(Edge(1, 3)).is_ok());
        assert!(graph.add_edge(Edge(1, 4)).is_ok());
        assert!(graph.add_edge(Edge(1, 11)).is_err());
        assert_eq!(graph.adj(1), [2, 3, 4]);
    }
}
