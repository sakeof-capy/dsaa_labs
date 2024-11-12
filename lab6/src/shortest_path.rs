use crate::graph::{Graph, Vertex, NO_EDGE_WEIGHT, NO_LOOP_WEIGHT};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoShortestPath,
    InvalidVertices(Vec<Vertex>),
}

pub fn floyd_shortest_path<const V: usize>(
    graph: &Graph<V>,
    v1: Vertex,
    v2: Vertex,
) -> Result<Vec<Vertex>, Error> {
    let invalid_vertices = vec![v1, v2]
        .into_iter()
        .filter(|v| !Graph::<V>::is_valid_vertex(*v))
        .collect::<Vec<_>>();

    if !invalid_vertices.is_empty() {
        return Err(Error::InvalidVertices(invalid_vertices));
    }

    let adj_matrix = graph.adjacency_matrix();
    let mut dist = [[NO_EDGE_WEIGHT; V]; V];
    let mut next = [[Option::<Vertex>::None; V]; V];

    for v1 in 0..V {
        for v2 in 0..V {
            if adj_matrix[v1][v2] != NO_EDGE_WEIGHT {
                dist[v1][v2] = adj_matrix[v1][v2];
                next[v1][v2] = Some(v2);
            }

            dist[v1][v1] = NO_LOOP_WEIGHT;
            next[v1][v1] = Some(v1);
        }
    }

    for k in 0..V {
        for i in 0..V {
            for j in 0..V {
                if dist[i][k] != NO_EDGE_WEIGHT
                    && dist[k][j] != NO_EDGE_WEIGHT
                    && dist[i][j] > dist[i][k] + dist[k][j]
                {
                    dist[i][j] = dist[i][k] + dist[k][j];
                    next[i][j] = next[i][k];
                }
            }
        }
    }

    if next[v1][v2].is_none() {
        return Err(Error::NoShortestPath);
    }

    let mut shortest_path = vec![v1];

    while let Some(&current_v) = shortest_path.last()
        && current_v != v2
    {
        if let Some(next_v) = next[current_v][v2] {
            shortest_path.push(next_v);
        } else {
            return Err(Error::NoShortestPath);
        }
    }

    Ok(shortest_path)
}
