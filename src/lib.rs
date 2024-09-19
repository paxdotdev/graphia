use std::sync::{Arc, Mutex, Weak};

/// A node in the graph, containing a value and a list of neighbors.
pub struct Node<T> {
    value: T,
    neighbors: Mutex<Vec<Weak<Mutex<Node<T>>>>>,
}

/// The graph structure, containing a list of nodes.
pub struct Graph<T> {
    nodes: Vec<Arc<Mutex<Node<T>>>>,
}

impl<T> Graph<T> {
    /// Creates a new, empty graph.
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    /// Adds a node with the given value to the graph.
    /// Returns the index of the new node.
    pub fn add_node(&mut self, value: T) -> usize {
        let node = Arc::new(Mutex::new(Node {
            value,
            neighbors: Mutex::new(Vec::new()),
        }));
        self.nodes.push(node);
        self.nodes.len() - 1 // Return the index of the new node
    }

    /// Adds an edge from the node at `from_index` to the node at `to_index`.
    pub fn add_edge(&self, from_index: usize, to_index: usize) {
        if from_index >= self.nodes.len() || to_index >= self.nodes.len() {
            panic!("Node index out of bounds");
        }
        let from_node = &self.nodes[from_index];
        let to_node = &self.nodes[to_index];

        // Lock the mutex to access neighbors
        let binding = from_node.lock().unwrap();
        let mut from_node_neighbors = binding.neighbors.lock().unwrap();
        from_node_neighbors.push(Arc::downgrade(to_node));
    }

    /// Returns a clone of the value of the node at the given index.
    pub fn get_node_value(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.nodes.get(index).map(|node| node.lock().unwrap().value.clone())
    }

    /// Returns a vector of clones of the values of the neighbors of the node at the given index.
    pub fn neighbors_of(&self, index: usize) -> Option<Vec<T>>
    where
        T: Clone,
    {
        if index >= self.nodes.len() {
            return None;
        }
        let node = &self.nodes[index];
        let node_guard = node.lock().unwrap();
        let neighbors = node_guard.neighbors.lock().unwrap();
        let neighbor_values = neighbors
            .iter()
            .filter_map(|weak_neighbor| weak_neighbor.upgrade())
            .map(|neighbor_arc| neighbor_arc.lock().unwrap().value.clone())
            .collect();
        Some(neighbor_values)
    }
}

impl<T> Drop for Graph<T> {
    fn drop(&mut self) {
        // Clear the nodes vector to drop all Arc<Node<T>> references
        self.nodes.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_add_node_and_get_value() {
        let mut graph = Graph::new();
        let index = graph.add_node(42);
        assert_eq!(graph.get_node_value(index), Some(42));
    }

    #[test]
    fn test_add_edge_and_neighbors() {
        let mut graph = Graph::new();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");
        graph.add_edge(node_a, node_b);
        let neighbors = graph.neighbors_of(node_a).unwrap();
        assert_eq!(neighbors, vec!["B"]);
    }

    #[test]
    fn test_cycle() {
        let mut graph = Graph::new();
        let node_a = graph.add_node("A");
        let node_b = graph.add_node("B");
        let node_c = graph.add_node("C");

        graph.add_edge(node_a, node_b);
        graph.add_edge(node_b, node_c);
        graph.add_edge(node_c, node_a); // Creates a cycle

        let neighbors_a = graph.neighbors_of(node_a).unwrap();
        assert_eq!(neighbors_a, vec!["B"]);

        let neighbors_b = graph.neighbors_of(node_b).unwrap();
        assert_eq!(neighbors_b, vec!["C"]);

        let neighbors_c = graph.neighbors_of(node_c).unwrap();
        assert_eq!(neighbors_c, vec!["A"]);
    }

    #[test]
    fn test_nonexistent_node() {
        let graph: Graph<i32> = Graph::new();
        assert_eq!(graph.get_node_value(0), None);
        assert_eq!(graph.neighbors_of(0), None);
    }

    #[test]
    #[should_panic(expected = "Node index out of bounds")]
    fn test_add_edge_invalid_indices() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_edge(0, 1); // There is no node at index 1
    }
}
