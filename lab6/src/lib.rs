#![feature(let_chains)]

pub mod graph;
pub mod shortest_path;

#[cfg(test)]
mod tests {
    use crate::graph::{Edge, Graph, NO_EDGE_WEIGHT, NO_LOOP_WEIGHT};
    use crate::shortest_path::{floyd_shortest_path, Error};

    #[test]
    fn test_graph() {
        let mut graph = Graph::<10>::default();
        assert!(graph.add_edge(Edge(1, 2, 10)).is_ok());
        assert!(graph.add_edge(Edge(1, 3, 11)).is_ok());
        assert!(graph.add_edge(Edge(1, 4, 12)).is_ok());
        assert!(graph.add_edge(Edge(1, 11, 13)).is_err());
        assert_eq!(graph.adj(1), [(2, 10), (3, 11), (4, 12)]);
    }

    #[test]
    fn test_adjacency() {
        const V: usize = 5;
        let mut graph = Graph::<V>::default();
        let adj_matrix = graph.adjacency_matrix();

        #[rustfmt::skip]
        let expected_adj_matrix = [
            [NO_LOOP_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT],
            [NO_EDGE_WEIGHT, NO_LOOP_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT],
            [NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_LOOP_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT],
            [NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_LOOP_WEIGHT, NO_EDGE_WEIGHT],
            [NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_LOOP_WEIGHT],
        ];

        assert_eq!(adj_matrix, expected_adj_matrix);

        assert!(graph.add_edge(Edge(1, 2, 10)).is_ok());
        assert!(graph.add_edge(Edge(1, 3, 11)).is_ok());
        assert!(graph.add_edge(Edge(1, 4, 12)).is_ok());
        assert!(graph.add_edge(Edge(2, 4, 13)).is_ok());

        let adj_matrix = graph.adjacency_matrix();

        #[rustfmt::skip]
        let expected_adj_matrix = [
            [NO_LOOP_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT, NO_EDGE_WEIGHT],
            [NO_EDGE_WEIGHT, NO_LOOP_WEIGHT,             10,             11,             12],
            [NO_EDGE_WEIGHT,             10, NO_LOOP_WEIGHT, NO_EDGE_WEIGHT,             13],
            [NO_EDGE_WEIGHT,             11, NO_EDGE_WEIGHT, NO_LOOP_WEIGHT, NO_EDGE_WEIGHT],
            [NO_EDGE_WEIGHT,             12,             13, NO_EDGE_WEIGHT, NO_LOOP_WEIGHT],
        ];

        assert_eq!(adj_matrix, expected_adj_matrix);
    }

    #[test]
    fn test_shortest_path_direct_edge() {
        let mut graph = Graph::<2>::default();
        assert!(graph.add_edge(Edge(0, 1, 5)).is_ok());
        let result = floyd_shortest_path(&graph, 0, 1);
        assert_eq!(result, Ok(vec![0, 1]));
    }

    #[test]
    fn test_shortest_path_with_intermediate_vertex() {
        let mut graph = Graph::<3>::default();
        assert!(graph.add_edge(Edge(0, 1, 3)).is_ok());
        assert!(graph.add_edge(Edge(1, 2, 4)).is_ok());
        let result = floyd_shortest_path(&graph, 0, 2);
        assert_eq!(result, Ok(vec![0, 1, 2]));
    }

    #[test]
    fn test_no_path_between_disconnected_vertices() {
        let graph = Graph::<2>::default();
        let result = floyd_shortest_path(&graph, 0, 1);
        assert_eq!(result, Err(Error::NoShortestPath));
    }

    #[test]
    fn test_invalid_vertices() {
        let graph = Graph::<2>::default();
        let result = floyd_shortest_path(&graph, 0, 3);
        assert_eq!(result, Err(Error::InvalidVertices(vec![3])));
    }

    #[test]
    fn test_large_graph_with_multiple_paths() {
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

        let result = floyd_shortest_path(&graph, 0, 9);
        assert_eq!(result, Ok(vec![0, 1, 2, 5, 6, 9]));
    }
}
