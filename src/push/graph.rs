use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::state::PushState;
use crate::push::vector::IntVector;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static NODE_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Debug, Hash, Eq)]
pub struct Node {
    node_id: usize,
    pre: i32,
    post: i32,
    updated: bool,
}

impl Node {
    pub fn new(state: i32) -> Self {
        Self {
            node_id: NODE_COUNTER.fetch_add(1, Ordering::Relaxed),
            pre: state,
            post: state,
            updated: false,
        }
    }

    /// Returns the difference between this node and the
    /// argument node as a String or None if they are
    /// identical.
    pub fn diff(&self, other: &Node) -> Option<String> {
        if self.node_id == other.get_id() &&
            self.pre == other.get_pre_state() &&
            self.post == other.get_post_state() &&
            self.updated == other.is_updated() {
                None
            } else {
               let mut diff_string: String = "N[".to_owned();
               diff_string.push_str("ID: ");
               diff_string.push_str(&self.node_id.to_string());
               diff_string.push_str(", ");
               if self.pre != other.get_pre_state() || self.post != other.get_post_state() {
                  diff_string.push_str(&self.pre.to_string());
                  diff_string.push_str("/");
                  diff_string.push_str(&other.get_pre_state().to_string());
                  diff_string.push_str(" <= STATE => ");
                  diff_string.push_str(&self.post.to_string());
                  diff_string.push_str("/");
                  diff_string.push_str(&other.get_post_state().to_string());
                  diff_string.push_str(", ");
               }
               if self.updated != other.is_updated() {
                  diff_string.push_str(&self.updated.to_string());
                  diff_string.push_str(" <= UPDATED => ");
                  diff_string.push_str(&other.is_updated().to_string());
                  diff_string.push_str(", ");
               }
               diff_string = diff_string.trim_end_matches(", ").to_string();
               diff_string.push_str("]");
               Some(diff_string)
        }
    }

    pub fn get_id(&self) -> usize {
        self.node_id
    }

    pub fn get_pre_state(&self) -> i32 {
        self.pre
    }


    pub fn get_post_state(&self) -> i32 {
       self.post
    }

    pub fn is_updated(&self) -> bool {
        self.updated
    }

    pub fn set_state(&mut self, state : i32) {
        self.pre = self.post;
        self.post = state;
        self.updated = true;
    }

}

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.node_id == other.node_id
        }
    }

    impl fmt::Display for Node {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut node_string: String = "".to_owned();
            node_string.push_str("ID: ");
            node_string.push_str(&self.node_id.to_string());
            node_string.push_str(", STATE: ");
            node_string.push_str(&self.pre.to_string());
            node_string.push_str("/");
            node_string.push_str(&self.post.to_string());
            node_string.push_str(", UPDATED: ");
            node_string.push_str(&self.updated.to_string());
            write!(
                f,
                "N[{}]",
                node_string
            )
        }
    }


    #[derive(Copy, Clone, Debug)]
    pub struct Edge {
        origin_node_id: usize,
        weight: f32,
    }

    impl Edge {
        pub fn new(node_id: usize, weight: f32) -> Self {
            Self {
                origin_node_id: node_id,
                weight: weight,
            }
        }
        /// Returns the difference between this edge and the
        /// argument edge as a String or None if they are
        /// identical.
        pub fn diff(&self, other: &Edge) -> Option<String> {
            if self.origin_node_id == other.get_origin_id() &&
                self.weight == other.get_weight() {
                    None
                } else {
                   let mut diff_string: String = "[".to_owned();
                   diff_string.push_str("ONID: ");
                   diff_string.push_str(&other.get_origin_id().to_string());
                   diff_string.push_str(", ");
                   if self.weight != other.get_weight() {
                      diff_string.push_str(&self.weight.to_string());
                      diff_string.push_str(" <= WEIGHT => ");
                      diff_string.push_str(&other.get_weight().to_string());
                   }
                   diff_string.push_str("]");
                   Some(diff_string)
            }
        }

        pub fn get_origin_id(&self) -> usize {
            self.origin_node_id
        }

        pub fn get_weight(&self) -> f32 {
            self.weight
        }
    }

    impl PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            self.origin_node_id == other.origin_node_id
        }
    }

    impl Hash for Edge {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.origin_node_id.hash(hasher);
        }
    }

    impl Eq for Edge {}

    impl fmt::Display for Edge {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut edge_string: String = "".to_owned();
                edge_string.push_str("ONID: ");
                edge_string.push_str(&self.origin_node_id.to_string());
                edge_string.push_str(", WEIGHT: ");
                edge_string.push_str(&self.weight.to_string());
            write!(
                f,
                "[{}]",
                edge_string
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct Graph {
        // Incoming edge list
        pub edges: HashMap<usize, HashSet<Edge>>,
        // Nodes by Id
        pub nodes: HashMap<usize, Node>,
    }

    impl fmt::Display for Graph {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut node_string: String = "".to_owned();
            for (_, node) in self.nodes.iter() {
                node_string.push_str("\n");
                node_string.push_str(&node.to_string());
                node_string.push_str(", ");
            }
            node_string = node_string.trim_end_matches(", ").to_string();
            let mut edge_string: String = "".to_owned();
            for (node, edges) in self.edges.iter() {
                for e in edges.iter() {
                    edge_string.push_str("\n");
                    edge_string.push_str("E[");
                    edge_string.push_str(&node.to_string());
                    edge_string.push_str(" <= ");
                    edge_string.push_str(&e.to_string());
                    edge_string.push_str("]");
                    edge_string.push_str(", ");
                }
            }
            edge_string = edge_string.trim_end_matches(", ").to_string();
            write!(
                f,
                "\nNODES({}): {}\nEDGES({}): {}",
                &self.node_size(),
                node_string,
                &self.edge_size(),
                edge_string,
            )
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                edges: HashMap::new(),
                nodes: HashMap::new(),
            }
        }

        /// Returns the difference between two graphs as String
        /// or None if they are identical.
        pub fn diff(&self, other: &Graph) -> Option<String>{
            let mut node_changes = 0;
            let mut edge_changes = 0;
            // 1. Nodes
            // Left side
            let mut node_diff : String = "".to_owned();
            for (lk,lv) in self.nodes.iter() {
                if !other.nodes.contains_key(lk) {
                    // Node removed
                    node_diff.push_str("\n");
                    node_diff.push_str("-"); 
                    node_diff.push_str(&lv.to_string());
                    node_diff.push_str(",");
                    node_changes += 1;
                }
            }
            // Right side 
            for (rk, rv) in other.nodes.iter() {
            if !self.nodes.contains_key(rk) {
                // Node added
                node_diff.push_str("\n");
                node_diff.push_str("+");
                node_diff.push_str(&rv.to_string());
                node_diff.push_str(",");
                node_changes += 1;
            } else {
               // Difference
               let left_node = self.nodes.get(rk).unwrap();
               let right_node = other.nodes.get(rk).unwrap();
               if let Some(diff) = left_node.diff(&right_node) {
                    node_diff.push_str("\n");
                    node_diff.push_str("~");
                    node_diff.push_str(&diff);
                    node_diff.push_str(",");
                    node_changes += 1;
                 }
              }

           }
           node_diff = node_diff.trim_end_matches(",").to_string();
           let mut edge_diff : String = "".to_owned();
           // 2. Edges
           // Left side
           for (lk,lv) in self.edges.iter() {
                if !other.edges.contains_key(lk) {
                    // All edges of this node are removed
                    if let Some(ies) = self.edges.get(lk) {
                        for ie in ies { 
                            edge_diff.push_str("\n");
                            edge_diff.push_str("-E[");
                            edge_diff.push_str(&lk.to_string());
                            edge_diff.push_str(" <= ");
                            edge_diff.push_str(&ie.to_string());
                            edge_diff.push_str("],");
                            edge_changes += 1;
                        }
                    }
                } else {
                    if let Some(ies) = self.edges.get(lk) {
                        for ie in ies { 
                            if !other.edges.get(lk).unwrap().contains(ie) {
                                // Edge removed
                                edge_diff.push_str("\n");
                                edge_diff.push_str("-E[");
                                edge_diff.push_str(&lk.to_string());
                                edge_diff.push_str(" <= ");
                                edge_diff.push_str(&ie.to_string());
                                edge_diff.push_str("],");
                                edge_changes += 1;
                            }
                        }
                    }
                }
           }
        for (rk,rv) in other.edges.iter() {
             if !self.edges.contains_key(rk) {
                 // All edges of this node are added
                 if let Some(ies) = other.edges.get(rk) {
                     for ie in ies { 
                         edge_diff.push_str("\n");
                         edge_diff.push_str("+E[");
                         edge_diff.push_str(&rk.to_string());
                         edge_diff.push_str(" <= ");
                         edge_diff.push_str(&ie.to_string());
                         edge_diff.push_str("],");
                         edge_changes += 1;
                     }
                 }
             } else {
                 if let Some(ies) = other.edges.get(rk) {
                     for rie in ies {
                         if !self.edges.get(rk).unwrap().contains(rie) {
                             // Edge added 
                             edge_diff.push_str("\n");
                             edge_diff.push_str("+E[");
                             edge_diff.push_str(&rk.to_string());
                             edge_diff.push_str(" <= ");
                             edge_diff.push_str(&rie.to_string());
                             edge_diff.push_str("],");
                             edge_changes += 1;
                         } else {
                            // Difference
                            let lie = self.edges.get(rk).unwrap().get(&Edge::new(rie.get_origin_id(),0.0)).unwrap();
                            if let Some(change) = lie.diff(rie) {
                                edge_diff.push_str("\n");
                                edge_diff.push_str("~E[");
                                edge_diff.push_str(&rk.to_string());
                                edge_diff.push_str(" <= ");
                                edge_diff.push_str(&change);
                                edge_diff.push_str("],");
                                edge_changes += 1;
                            }
                         }
                     }
                 }
             }
        }
        edge_diff = edge_diff.trim_end_matches(",").to_string();
        if node_changes + edge_changes == 0 {
            return None;
        } else {
            let mut diff_string: String = "\nNODES(".to_owned();
            diff_string.push_str(&node_changes.to_string());
            diff_string.push_str("):");
            diff_string.push_str(&node_diff);
            diff_string.push_str("\n");
            diff_string.push_str("EDGES(");
            diff_string.push_str(&edge_changes.to_string());
            diff_string.push_str("):");
            diff_string.push_str(&edge_diff);
            return Some(diff_string);
           }
        }

        /// Adds an new node with the given state and activity
        /// and returns its assigned IDs.
        pub fn add_node(&mut self, state: i32) -> usize {
            let node = Node::new(state);
            let node_id = node.node_id;
            self.nodes.insert(node_id, node);
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
                } else {
                    let mut new_incoming_edges = HashSet::new();
                    new_incoming_edges.insert(Edge::new(origin_id, weight));
                    self.edges.insert(destination_id, new_incoming_edges);
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

        pub fn node_is_updated(&self, id: &usize) -> Option<bool> {
           if let Some(node) = self.nodes.get(&id) {
               Some(node.updated)
           } else {
               None
           }
        }

        /// Resets updated flag of all nodes
        pub fn reset_update_flag(&mut self) {
            for (_, node) in self.nodes.iter_mut() {
                node.updated = false;
            }
        }

        /// Get the pre state of the node with the given ID.
       pub fn get_pre_state(&self, id: &usize) -> Option<i32> {
            if let Some(node) = self.nodes.get(&id) {
                Some(node.pre)
            } else {
                None
            }
        }

        /// Get the post state of the node with the given ID.
        pub fn get_post_state(&self, id: &usize) -> Option<i32> {
            if let Some(node) = self.nodes.get(&id) {
                Some(node.post)
            } else {
                None
            }
        }

        /// Set the state of the node with the given ID. Update
        /// previous state and update flag.
        pub fn set_state(&mut self, id: &usize, state: i32) {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.set_state(state);
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
            String::from("GRAPH.NODE*RESETUPDATE"),
            Instruction::new(graph_node_reset_update),
        );
        map.insert(
            String::from("GRAPH.NODE*PRE"),
            Instruction::new(graph_node_get_pre_state),
        );
        map.insert(
            String::from("GRAPH.NODE*POST"),
            Instruction::new(graph_node_get_post_state),
        );
        map.insert(
            String::from("GRAPH.NODE*UPDATED"),
            Instruction::new(graph_node_is_updated),
        );
        map.insert(
            String::from("GRAPH.NODE*PREDECESSORS"),
            Instruction::new(graph_node_predecessors),
        );
        map.insert(
            String::from("GRAPH.NODE*SUCCESSORS"),
            Instruction::new(graph_node_successors),
        );
        map.insert(
            String::from("GRAPH.NODE*SETSTATE"),
            Instruction::new(graph_node_set_state),
        );
        map.insert(
            String::from("GRAPH.EDGE*ADD"),
            Instruction::new(graph_edge_add),
        );
        map.insert(
            String::from("GRAPH.EDGE*GETWEIGHT"),
            Instruction::new(graph_edge_get_weight),
        );
        map.insert(
            String::from("GRAPH.EDGE*SETWEIGHT"),
            Instruction::new(graph_edge_set_weight),
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
                if let Some(state) = push_state.int_stack.pop() {
                    push_state
                        .int_stack
                        .push(graph.add_node(state) as i32);
                }
        }
    }

    /// GRAPH.NODE*RESETUPDATE: Reset the 'Updated' flag of each node of the top graph item.
    fn graph_node_reset_update(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            graph.reset_update_flag();
        }
    }


    /// GRAPH.NODE*PRE: Pushes the pre update state of the node the with the specified id to the
    /// integer stack. 
    fn graph_node_get_pre_state(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(id) = push_state.int_stack.pop() {
                if id > 0 {
                    if let Some(state) = graph.get_pre_state(&(id as usize)) {
                        push_state.int_stack.push(state);
                    }
                }
            }
        }
    }

    /// GRAPH.NODE*POST: Pushes the post update state of the node with the specified id to the
    /// INTEGER stack.
    fn graph_node_get_post_state(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(id) = push_state.int_stack.pop() {
                if id > 0 {
                    if let Some(state) = graph.get_post_state(&(id as usize)) {
                        push_state.int_stack.push(state);
                    }
                }
            }
        }
    }

    /// GRAPH.NODE*UPDATED: Pushes the update flag of the node with the specified id to the
    /// BOOLEAN stack.
    fn graph_node_is_updated(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(id) = push_state.int_stack.pop() {
                if id > 0 {
                    if let Some(is_updated) = graph.node_is_updated(&(id as usize)) {
                        push_state.bool_stack.push(is_updated);
                    }
                }
            }
        }
    }

    /// GRAPH.NODE*SETSTATE: Sets the state for the node with the specified id where the
    /// new state and the id are the first and second element of the stack.
    /// If the id does not exist this acts as NOOP.
    fn graph_node_set_state(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(state) = push_state.int_stack.pop() {
                if let Some(id) = push_state.int_stack.pop() {
                    if id > 0 {
                        graph.set_state(&(id as usize), state);
                    }
                }
            }
        }
    }

    /// GRAPH.EDGE*ADD: Adds a new edge to the graph on top of the GRAPH stack.
    fn graph_edge_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(weight) = push_state.float_stack.pop() {
                if let Some(ids) = push_state.int_stack.pop_vec(2) {
                    let origin_id = ids[0] as usize;       // Second element
                    let destination_id = ids[1] as usize; // Top element
                    graph.add_edge(origin_id, destination_id, weight);
                }
            }
        }
    }

    /// GRAPH.NODE*PREDECESORS: Pushes the IDs of all nodes with an edge pointing to the current
    /// node to the INTVECTOR stack. The ID of current node is taken from the INTEGER stack.
    fn graph_node_predecessors(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get(0) {
            if let Some(node_id) = push_state.int_stack.pop() {
                if node_id > 0 {
                    let mut predecessors = vec![];
                    if let Some(incoming_edges) = graph.edges.get(&(node_id as usize)) {
                        for edge in incoming_edges {
                            predecessors.push(edge.origin_node_id as i32);
                        }
                    }
                    push_state
                        .int_vector_stack
                        .push(IntVector::new(predecessors));
                }
            }
        }
    }

    /// GRAPH.NODE*SUCCESSORS: Pushes the IDs of all nodes with an edge pointing to the current
    /// node to the INTVECTOR stack. The ID of current node is taken from the INTEGER stack.
    fn graph_node_successors(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get(0) {
            if let Some(node_id) = push_state.int_stack.pop() {
                if node_id > 0 {
                    let mut successors = vec![];
                    for (k,v) in graph.edges.iter() {
                        if v.contains(&Edge::new(node_id as usize,0.0)) {
                            successors.push(*k as i32);
                        }
                    }
                    push_state
                        .int_vector_stack
                        .push(IntVector::new(successors));
                }
            }
        }
    }

    /// GRAPH.EDGE*GETWEIGHT: Gets the weight for the edge with the specified origin and 
    /// destination id.
    fn graph_edge_get_weight(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
                if let Some(ids) = push_state.int_stack.pop_vec(2) {
                    let origin_id = ids[0] as usize;
                    let destination_id = ids[1] as usize;
                    if let Some(weight) = graph.get_weight(&origin_id, &destination_id) {
                        push_state.float_stack.push(weight);
                    }
                }
            }
        }

    /// GRAPH.EDGE*SETWEIGHT: Sets the weight for the edge with the specified origin and 
    /// destination id.
    fn graph_edge_set_weight(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(weight) = push_state.float_stack.pop() {
                if let Some(ids) = push_state.int_stack.pop_vec(2) {
                    let origin_id = ids[0] as usize;
                    let destination_id = ids[1] as usize;
                    graph.set_weight(&origin_id, &destination_id, weight);
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

    pub fn test_node(test_state: &mut PushState, state: i32) -> i32 {
        test_state.int_stack.push(state);
        graph_node_add(test_state, &icache());
        test_state.int_stack.pop().unwrap()
    }

    pub fn test_edge(test_state: &mut PushState, origin_id: i32, destination_id: i32, weight: f32) {
        test_state.int_stack.push(origin_id);      // Second element
        test_state.int_stack.push(destination_id); // Top element
        test_state.float_stack.push(weight);
        graph_edge_add(test_state, &icache());
    }

    #[test]
    fn graph_node_predecessors_are_pushed() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let origin_id = test_node(&mut test_state, 1);
        let origin_id2 = test_node(&mut test_state, 1);
        let destination_id = test_node(&mut test_state, 1);
        test_edge(&mut test_state, origin_id, destination_id, 0.1);
        test_edge(&mut test_state, origin_id2, destination_id, 0.1);
        test_state.int_stack.push(destination_id);
        graph_node_predecessors(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        let predecessors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(predecessors.len(), 2);
        assert!(predecessors.contains(&origin_id));
        assert!(predecessors.contains(&origin_id2));
    }

    #[test]
    fn graph_node_successors_are_pushed() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let origin_id = test_node(&mut test_state, 1);
        let destination_id = test_node(&mut test_state, 1);
        let destination_id2 = test_node(&mut test_state, 1);
        test_edge(&mut test_state, origin_id, destination_id, 0.1);
        test_edge(&mut test_state, origin_id, destination_id2, 0.1);
        test_state.int_stack.push(origin_id);
        graph_node_successors(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        let successors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(successors.len(), 2);
        assert!(successors.contains(&destination_id));
        assert!(successors.contains(&destination_id2));

    }

    #[test]
    fn graph_node_add_updates_graph() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let node_id = test_node(&mut test_state, 1) as usize;
        assert_eq!(test_state.graph_stack.get(0).unwrap().node_size(), 1);
        assert_eq!(test_state.graph_stack.get(0).unwrap().edge_size(), 0);
        assert_eq!(
            test_state
                .graph_stack
                .get(0)
                .unwrap()
                .get_post_state(&node_id)
                .unwrap(),
            1
        );
    }

    #[test]
    fn graph_node_state_modification() {
        let mut test_state = PushState::new();
        let node_state_1 = 94;
        let node_state_2 = 123;
        graph_add(&mut test_state, &icache());
        let node_id = test_node(&mut test_state, node_state_1);
        test_state.int_stack.push(node_id.clone() as i32);
        graph_node_get_post_state(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), node_state_1);
        test_state.int_stack.push(node_id.clone() as i32);
        test_state.int_stack.push(node_state_2);
        graph_node_set_state(&mut test_state, &icache());
        assert_eq!(
            test_state
                .graph_stack
                .get(0)
                .unwrap()
                .get_pre_state(&(node_id as usize))
                .unwrap(),
           node_state_1
        );
        assert_eq!(
            test_state
                .graph_stack
                .get(0)
                .unwrap()
                .get_post_state(&(node_id as usize))
                .unwrap(),
          node_state_2
        );
        assert!(test_state.graph_stack.get(0).unwrap().node_is_updated(&(node_id as usize)).unwrap());
    }

    #[test]
    fn graph_edge_add_updates_graph() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let origin_id = test_node(&mut test_state, 1);
        let destination_id = test_node(&mut test_state, 1);
        test_edge(&mut test_state, origin_id, destination_id, 0.1);
        assert_eq!(test_state.graph_stack.get(0).unwrap().node_size(), 2);
        assert_eq!(test_state.graph_stack.get(0).unwrap().edge_size(), 1);
        println!("{}", test_state.to_string());
        println!("oid = {}, did = {}",origin_id, destination_id);
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

    #[test]
    fn graph_print_differences() {
        let mut test_graph = Graph::new();
        let mut test_ids = vec![];
        test_ids.push(test_graph.add_node(1));
        test_ids.push(test_graph.add_node(2));
        test_ids.push(test_graph.add_node(3));
        test_ids.push(test_graph.add_node(4));
       
        test_graph.add_edge(test_ids[1], test_ids[0], 1.3);
        test_graph.add_edge(test_ids[2], test_ids[0], 1.6);
        test_graph.add_edge(test_ids[3], test_ids[0], 1.5);
        
        let mut changed_test_graph = test_graph.clone();
        test_ids.push(changed_test_graph.add_node(5));
        changed_test_graph.add_edge(test_ids[4], test_ids[0], 1.2);
        changed_test_graph.set_state(&test_ids[1], 99);
        changed_test_graph.set_weight(&test_ids[1], &test_ids[0], 0.2);
        let diff = test_graph.diff(&changed_test_graph).unwrap();
        assert!(diff.contains("NODES(2)"));
        assert!(diff.contains("~N[ID: 12, 2/2 <= STATE => 2/99, false <= UPDATED => true]"));
        assert!(diff.contains("+N[ID: 15, STATE: 5/5, UPDATED: false]"));
        assert!(diff.contains("EDGES(2)"));
        assert!(diff.contains("+E[11 <= [ONID: 15, WEIGHT: 1.2]]"));
        assert!(diff.contains("~E[11 <= [ONID: 12, 1.3 <= WEIGHT => 0.2]]"));

    }

}
