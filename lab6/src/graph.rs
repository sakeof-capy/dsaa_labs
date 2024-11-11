pub type Vertex = usize;

pub struct Edge(pub Vertex, pub Vertex);

pub enum Error {
    InvalidVertices,
}

pub struct Graph<const V: usize> {
    lists: [Vec<Vertex>; V],
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
        let v1_is_valid = Self::is_valid_vertex(v1);
        let v2_is_valid = Self::is_valid_vertex(v2);

        if !v1_is_valid || !v2_is_valid {
            return Err(Error::InvalidVertices);
        }

        self.lists[v1].push(v2);
        self.lists[v2].push(v1);

        Ok(())
    }

    pub fn adj(&self, v: usize) -> &[Vertex] {
        &self.lists[v]
    }

    fn is_valid_vertex(v: usize) -> bool {
        v < V
    }
}
