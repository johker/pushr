use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static NODE_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Debug, Hash, Eq)]
pub struct Node {
    node_id: usize,
    state: i32,
    active: bool,
}

impl Node {
    pub fn new(state: i32, active: bool) -> Self {
        Self {
            node_id: NODE_COUNTER.fetch_add(1, Ordering::Relaxed),
            state: state,
            active: active,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    node_id: usize,
    weight: f32,
}

impl Edge {
    pub fn new(node_id: usize, weight: f32) -> Self {
        Self {
            node_id: node_id,
            weight: weight,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
    }
}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.node_id.hash(hasher);
    }
}

impl Eq for Edge {}

pub struct Graph {
    // Incoming edge list
    edges: HashMap<usize, HashSet<Edge>>,
    // Nodes by Id
    nodes: HashMap<usize, Node>,
}

impl Graph {
    /// Adds an new node with the given state and activity
    /// and returns its assigned IDs.
    pub fn add_node(&mut self, state: i32, active: bool) -> usize {
        let node = Node::new(state, active);
        let node_id = node.node_id;
        self.nodes.insert(node_id, node);
        self.edges.insert(node_id.clone(), HashSet::new());
        node_id.clone()
    }

    /// Removesthe node with the given id and
    /// all its outgoing connections from the graph.
    pub fn remove_node(&mut self, id: usize) {
        self.nodes.remove(&id);
        self.edges.remove(&id);
        for (_, edges) in self.edges.iter_mut() {
            edges.remove(&Edge::new(id, 0.0));
        }
    }

    /// Adds a connection to the directed graph from the
    /// node with origin_id to destination_id and assigns the
    /// weight parameter to it.
    pub fn add_connection(&mut self, origin_id: usize, destination_id: usize, weight: f32) {
        if self.nodes.contains_key(&origin_id) && self.nodes.contains_key(&destination_id) {
            if let Some(incoming_edges) = self.edges.get_mut(&destination_id) {
                // Use origin_id to create an incoming edge
                let edge = Edge::new(origin_id, weight);
                incoming_edges.insert(edge);
            }
        }
    }

    /// Removes the connection from origin_id to destination_id
    /// from the graph
    pub fn remove_connection(&mut self, origin_id: usize, destination_id: usize) {
        if let Some(incoming_edges) = self.edges.get_mut(&destination_id) {
            incoming_edges.remove(&Edge::new(origin_id, 0.0));
        }
    }
}
