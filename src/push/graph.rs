use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::state::PushState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
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

#[derive(Clone, Debug)]
pub struct Graph {
    // Incoming edge list
    edges: HashMap<usize, HashSet<Edge>>,
    // Nodes by Id
    nodes: HashMap<usize, Node>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GRAPH [{};{}]", &self.node_size(), &self.edge_size())
    }
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    /// Adds an new node with the given state and activity
    /// and returns its assigned IDs.
    pub fn add_node(&mut self, state: i32, active: bool) -> usize {
        let node = Node::new(state, active);
        let node_id = node.node_id;
        self.nodes.insert(node_id, node);
        self.edges.insert(node_id.clone(), HashSet::new());
        node_id.clone()
    }

    /// Removes the node with the given id and
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
    pub fn add_edge(&mut self, origin_id: usize, destination_id: usize, weight: f32) {
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
    pub fn remove_edge(&mut self, origin_id: usize, destination_id: usize) {
        if let Some(incoming_edges) = self.edges.get_mut(&destination_id) {
            incoming_edges.remove(&Edge::new(origin_id, 0.0));
        }
    }

    /// Returns all nodes that are in the given state.
    pub fn get_nodes_with_state(&self, state: &i32) -> Vec<usize> {
        let mut nodes = vec![];
        for (_, node) in self.nodes.iter() {
            if node.state == *state {
                nodes.push(node.node_id);
            }
        }
        nodes
    }

    pub fn get_active_nodes_with_state(&self, active: &bool, state: &i32) -> Vec<usize> {
        let mut nodes = vec![];
        for (_, node) in self.nodes.iter() {
            if node.state == *state && node.active == *active {
                nodes.push(node.node_id);
            }
        }
        nodes
    }

    /// Get the state of the node with the given ID.
    pub fn get_state(&self, id: &usize) -> Option<i32> {
        if let Some(node) = self.nodes.get(&id) {
            Some(node.state)
        } else {
            None
        }
    }

    /// Set the state of the node with the given ID.
    pub fn set_state(&mut self, id: usize, state: i32) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.state = state;
        }
    }

    /// Get the active flag of the node with the given ID.
    pub fn get_active(&self, id: &usize) -> Option<bool> {
        if let Some(node) = self.nodes.get(&id) {
            Some(node.active)
        } else {
            None
        }
    }

    /// Set the active flag of the node with the given ID.
    pub fn set_active(&mut self, id: usize, active: bool) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.active = active;
        }
    }

    /// Get the weight of the edge between the nodes with
    /// origin_id and destination_id.
    pub fn get_weight(&self, origin_id: &usize, destination_id: &usize) -> Option<f32> {
        if let Some(incoming_edges) = self.edges.get(&destination_id) {
            if let Some(edge) = incoming_edges.get(&Edge::new(*origin_id, 0.0)) {
                return Some(edge.weight);
            }
        }
        None
    }

    /// Set the weight of the edge between the nodes with
    /// origin_id and destination_id.
    pub fn set_weight(&mut self, origin_id: &usize, destination_id: &usize, weight: f32) {
        if let Some(incoming_edges) = self.edges.get_mut(&destination_id) {
            incoming_edges.replace(Edge::new(*origin_id, weight));
        }
    }

    /// Returns the number of nodes
    pub fn node_size(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges
    pub fn edge_size(&self) -> usize {
        let mut num_edges = 0;
        for (_k, v) in self.edges.iter() {
            num_edges += v.len();
        }
        num_edges
    }
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.nodes == other.nodes && self.edges == other.edges
    }
}

pub fn load_graph_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("GRAPH.ADD"), Instruction::new(graph_add));
    map.insert(
        String::from("GRAPH.NODE*ADD"),
        Instruction::new(graph_node_add),
    );
    map.insert(
        String::from("GRAPH.EDGE*ADD"),
        Instruction::new(graph_edge_add),
    );
}

/// GRAPH.ADD: Pushes a new instance of an empty graph to the graph stack.
fn graph_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.graph_stack.push(Graph::new());
}

/// GRAPH.NODE*ADD: Adds a new node to the graph on top of the GRAPH stack. The ID
/// of the node is pushed to the INTEGER stack.
fn graph_node_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(graph) = push_state.graph_stack.get_mut(0) {
        if let Some(active_flag) = push_state.bool_stack.pop() {
            if let Some(state) = push_state.int_stack.pop() {
                push_state
                    .int_stack
                    .push(graph.add_node(state, active_flag) as i32);
            }
        }
    }
}

/// GRAPH.EDGE*ADD: Adds a new edge to the graph on top of the GRAPH stack.
fn graph_edge_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(graph) = push_state.graph_stack.get_mut(0) {
        if let Some(weight) = push_state.float_stack.pop() {
            if let Some(ids) = push_state.int_stack.pop_vec(2) {
                let origin_id = ids[0] as usize;
                let destination_id = ids[1] as usize;
                graph.add_edge(origin_id, destination_id, weight);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn graph_node_add_updates_graph() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.int_stack.push(1);
        graph_add(&mut test_state, &icache());
        graph_node_add(&mut test_state, &icache());
        assert_eq!(test_state.graph_stack.get(0).unwrap().node_size(), 1);
        assert_eq!(test_state.graph_stack.get(0).unwrap().edge_size(), 0);
        let node_id = test_state.int_stack.pop().unwrap() as usize;
        assert_eq!(
            test_state
                .graph_stack
                .get(0)
                .unwrap()
                .get_state(&node_id)
                .unwrap(),
            1
        );
        assert_eq!(
            test_state
                .graph_stack
                .get(0)
                .unwrap()
                .get_active(&node_id)
                .unwrap(),
            true
        );
    }

    #[test]
    fn graph_edge_add_updates_graph() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        test_state.bool_stack.push(true);
        test_state.int_stack.push(1);
        graph_node_add(&mut test_state, &icache());
        let origin_id = test_state.int_stack.pop().unwrap();
        test_state.bool_stack.push(true);
        test_state.int_stack.push(1);
        graph_node_add(&mut test_state, &icache());
        let destination_id = test_state.int_stack.pop().unwrap();
        test_state.int_stack.push(origin_id);
        test_state.int_stack.push(destination_id);
        test_state.float_stack.push(0.1);
        graph_edge_add(&mut test_state, &icache());
        assert_eq!(test_state.graph_stack.get(0).unwrap().node_size(), 2);
        assert_eq!(test_state.graph_stack.get(0).unwrap().edge_size(), 1);
        assert_eq!(
            test_state
                .graph_stack
                .get(0)
                .unwrap()
                .get_weight(&(origin_id as usize), &(destination_id as usize))
                .unwrap(),
            0.1
        );
    }
}
