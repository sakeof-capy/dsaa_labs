pub type Vertex = usize;
pub type Weight = u32;

pub const NO_LOOP_WEIGHT: Weight = 0;
pub const NO_EDGE_WEIGHT: Weight = Weight::MAX;

pub struct Edge(pub Vertex, pub Vertex, pub Weight);

#[derive(Debug)]
pub enum Error {
    InvalidVertices,
}

pub struct Graph<const V: usize> {
    lists: [Vec<(Vertex, Weight)>; V],
}

impl<const V: usize> Default for Graph<V> {
    fn default() -> Self {
        Self {
            lists: core::array::from_fn(|_| Vec::default()),
        }
    }
}

impl<const V: usize> Graph<V> {
    pub fn add_edge(&mut self, edge: Edge) -> Result<(), Error> {
        let v1 = edge.0;
        let v2 = edge.1;
        let weight = edge.2;
        let v1_is_valid = Self::is_valid_vertex(v1);
        let v2_is_valid = Self::is_valid_vertex(v2);

        if !v1_is_valid || !v2_is_valid {
            return Err(Error::InvalidVertices);
        }

        self.lists[v1].push((v2, weight));
        self.lists[v2].push((v1, weight));

        Ok(())
    }

    pub fn adjacency_matrix(&self) -> [[Weight; V]; V] {
        let mut matrix = [[NO_EDGE_WEIGHT; V]; V];

        for (v1, v1_adj) in self.lists.iter().enumerate() {
            for (v2, weight) in v1_adj.iter() {
                matrix[v1][*v2] = *weight;
                matrix[*v2][v1] = *weight;
            }
        }

        for v in 0..V {
            matrix[v][v] = NO_LOOP_WEIGHT;
        }

        matrix
    }

    pub fn adj(&self, v: usize) -> &[(Vertex, Weight)] {
        &self.lists[v]
    }

    pub fn is_valid_vertex(v: usize) -> bool {
        v < V
    }
}
