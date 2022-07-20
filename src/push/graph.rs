use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::state::PushState;
use crate::push::stack::PushPrint;
use crate::push::vector::IntVector;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

static NODE_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Clone, Debug, Hash, Eq)]
pub struct Node {
    node_id: usize,
    state: i32,
}

impl Node {
    pub fn new(state: i32) -> Self {
        Self {
            node_id: NODE_COUNTER.fetch_add(1, Ordering::Relaxed),
            state: state,
        }
    }

    /// Returns the difference between this node and the
    /// argument node as a String or None if they are
    /// identical.
    pub fn diff(&self, other: &Node) -> Option<String> {
        if self.node_id == other.get_id() &&
            self.state == other.get_state() {
                None
            } else {
               let mut diff_string: String = "N[".to_owned();
               diff_string.push_str("ID: ");
               diff_string.push_str(&self.node_id.to_string());
               diff_string.push_str(", ");
               if self.state != other.get_state() {
                  diff_string.push_str(&self.state.to_string());
                  diff_string.push_str(" <= STATE => ");
                  diff_string.push_str(&other.get_state().to_string());
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

    pub fn get_state(&self) -> i32 {
        self.state
    }

    pub fn set_state(&mut self, state : i32) {
        self.state = state;
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
            node_string.push_str(&self.state.to_string());
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

        pub fn set_weight(&mut self, weight : f32) {
            self.weight = weight;
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

    #[derive(Clone, Debug, Default)]
    pub struct Graph {
        // Incoming edge list
        pub edges: HashMap<usize, Vec<Edge>>,
        // Nodes by Id
        pub nodes: HashMap<usize, Node>,
    }

    impl PushPrint for Graph {
       fn to_pstring(&self) -> String {
           format!("{}", self.to_string())
       }
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
           for (lk,_lv) in self.edges.iter() {
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
        for (rk,_rv) in other.edges.iter() {
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
                            let lie = (*self.edges.get(rk).unwrap()).iter().find( |&&x| x == Edge::new(rie.get_origin_id(),0.0) ).unwrap();
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
                if let Some(edge_idx) = edges.iter().position(|x| x == &Edge::new(id, 0.0)) {
                    edges.remove(edge_idx);
                }
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
                    if !incoming_edges.contains(&edge){
                        incoming_edges.push(edge);
                    }
                } else {
                    let mut new_incoming_edges = vec![]; 
                    new_incoming_edges.push(Edge::new(origin_id, weight));
                    self.edges.insert(destination_id, new_incoming_edges);
                }
            }
        }

        /// Removes the connection from origin_id to destination_id
        /// from the graph
        pub fn remove_edge(&mut self, origin_id: usize, destination_id: usize) {
            if let Some(incoming_edges) = self.edges.get_mut(&destination_id) {
                incoming_edges.retain(|x| x != &Edge::new(origin_id, 0.0));
            }
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
        pub fn set_state(&mut self, id: &usize, state: i32) {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.set_state(state);
            }
        }


        /// Get the weight of the edge between the nodes with
        /// origin_id and destination_id.
        pub fn get_weight(&self, origin_id: &usize, destination_id: &usize) -> Option<f32> {
            if let Some(incoming_edges) = self.edges.get(&destination_id) {
                if let Some(edge_idx) = incoming_edges.iter().position(|x| x == &Edge::new(*origin_id, 0.0)) {
                   return Some(incoming_edges[edge_idx].get_weight()); 
                }
            }
            None
        }

        /// Set the weight of the edge between the nodes with
        /// origin_id and destination_id.
        pub fn set_weight(&mut self, origin_id: &usize, destination_id: &usize, weight: f32) {
            if let Some(incoming_edges) = self.edges.get_mut(&destination_id) {
                if let Some(edge_idx) = incoming_edges.iter().position(|x| x == &Edge::new(*origin_id, 0.0)) {
                    incoming_edges[edge_idx].set_weight(weight); 
                }
            }
        }

        /// Returns the number of nodes
        pub fn node_size(&self) -> usize {
            self.nodes.len()
        }

        /// Returns all nodes ids that contains the given state parameter
        pub fn filter(&self, states: &Vec<i32>) -> Vec<i32> {
            let mut filtered_nodes = vec![];
            for (_,n) in self.nodes.iter() {
                if states.len() == 0 {
                    filtered_nodes.push(n.get_id() as i32);
                } else {
                    for state in states.iter() {
                       if n.get_state() == *state {
                            filtered_nodes.push(n.get_id() as i32);
                        }
                    }
                }
            }
            filtered_nodes
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
        map.insert(String::from("GRAPH.DUP"), Instruction::new(graph_dup));
        map.insert(
            String::from("GRAPH.NODE*ADD"),
            Instruction::new(graph_node_add),
        );
        map.insert(
            String::from("GRAPH.NODE*GETSTATE"),
            Instruction::new(graph_node_get_state),
        );
        map.insert(
            String::from("GRAPH.NODE*HISTORY"),
            Instruction::new(graph_node_history),
        );
        map.insert(
            String::from("GRAPH.NODE*SETSTATE"),
            Instruction::new(graph_node_set_state),
            );
        map.insert(
            String::from("GRAPH.NODE*NEIGHBORS"),
            Instruction::new(graph_node_neighbors),
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
            String::from("GRAPH.NODE*STATESWITCH"),
            Instruction::new(graph_node_state_switch),
            );
        map.insert(
            String::from("GRAPH.NODES"),
            Instruction::new(graph_nodes),
        );
        map.insert(
            String::from("GRAPH.STACKDEPTH"),
            Instruction::new(graph_stack_depth),
        );
        map.insert(
            String::from("GRAPH.PRINT"),
            Instruction::new(graph_print),
            );
        map.insert(
            String::from("GRAPH.EDGE*ADD"),
            Instruction::new(graph_edge_add),
        );
        map.insert(
            String::from("GRAPH.EDGE*HISTORY"),
            Instruction::new(graph_edge_history),
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

    /// GRAPH.DUP: Duplicates the top item on the GRAPH stack.
    fn graph_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(gval) = push_state.graph_stack.copy(0) {
            push_state.graph_stack.push(gval);
        }
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

    
    /// GRAPH.NODE*STATESWITCH: Sets the state defined by the top two INTEGER items to the nodes 
    /// with the IDs specified by top item of the INTVECTOR stack. If the element at position i 
    /// of the top BOOLVECTOR item is true then the state of the node corresponding to the ID 
    /// at position i of the INTVECTOR is set to the second element, otherwise it is set to 
    /// the top element. 
    fn graph_node_state_switch(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(node_ids) = push_state.int_vector_stack.pop() {
                if let Some(state_switch) = push_state.bool_vector_stack.pop() {
                    if let Some(states) = push_state.int_stack.pop_vec(2) {
                        let on_state = states[0];
                        let off_state = states[1];
                        let switch_len = i32::max(i32::min(node_ids.values.len() as i32 , state_switch.values.len() as i32), 0) as usize;
                        for i in 0..switch_len {
                            if state_switch.values[i] {
                                graph.set_state(&(node_ids.values[i] as usize), on_state);
                            } else {
                                graph.set_state(&(node_ids.values[i] as usize), off_state);
                            }
                        }
                    }
                }
            }
        }
    }

    /// GRAPH.NODES: Pushes the IDs of the nodes that are in one of the predefined states 
    /// to the INTVECTOR stack. The states are taken from the top item 
    /// of the INTVECTOR stack. If the array is empty all node IDs of the graph are pushed. 
    fn graph_nodes(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get(0) {
            if let Some(states) = push_state.int_vector_stack.pop() {
                let pf = graph.filter(&states.values);
                    push_state.int_vector_stack.push(IntVector::new(pf)); 
                }
        }
    }

    /// GRAPH.NODE*GETSTATE: Pushes the state of the node the with the specified 
    /// id to the integer stack. 
    fn graph_node_get_state(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get_mut(0) {
            if let Some(id) = push_state.int_stack.pop() {
                if id > 0 {
                    if let Some(state) = graph.get_state(&(id as usize)) {
                        push_state.int_stack.push(state);
                    }
                }
            }
        }
    }

    /// GRAPH.NODE*HISTORY: Pushes the state of the node with the specified id and stack position
    /// to the integer stack. ID and position are the second and the top item of the INTEGER stack
    /// respectively.
    fn graph_node_history(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(pos) = push_state.int_stack.pop() {
            if pos >= 0 {
                if let Some(id) = push_state.int_stack.pop() {
                    if let Some(graph) = push_state.graph_stack.get_mut(pos as usize) {
                        if id >= 0 {
                            if let Some(state) = graph.get_state(&(id as usize)) {
                                push_state.int_stack.push(state);
                            }
                        }
                    }
                }
            }
        }
    }

    /// GRAPH.PRINT: Pushes a string representation of the top GRAPH stack item to 
    /// the name stack.
    fn graph_print(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get(0) {
            push_state.name_stack.push(graph.to_string());
        }
    }

    /// GRAPH.PRINT*DIFF: Pushes a string representation of the diff of the top to the second 
    /// item on the GRAPH stack to the name stack.
    fn graph_print_diff(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(new_graph) = push_state.graph_stack.get(0) {
            if let Some(old_graph) = push_state.graph_stack.get(1) {
                if let Some(diff) = old_graph.diff(new_graph) {
                    push_state.name_stack.push(diff.to_string());
                }
            }
        }
    }

    /// GRAPH.STACKDEPTH: Pushes the stack depth onto the INTEGER stack 
    pub fn graph_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        push_state
            .int_stack
            .push(push_state.graph_stack.size() as i32);
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

    /// GRAPH.NODE*NEIGHBORS: Pushes the IDs of the predecessor and successor nodes that are in
    /// one of the predefined states to the INTVECTOR stack. The states are taken from the top 
    /// item of the INTVECTOR stack. If the array is empty all neighbor node IDs are pushed. 
    /// The origin node id is taken from the INTEGER stack.
    fn graph_node_neighbors(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get(0) {
            if let Some(states) = push_state.int_vector_stack.pop() {
                if let Some(node_id) = push_state.int_stack.pop() {
                    if node_id > 0 {
                        let mut neighbors = vec![];
                        if let Some(incoming_edges) = graph.edges.get(&(node_id as usize)) {
                            for edge in incoming_edges {
                                if let Some(origin_state) = graph.get_state(&edge.origin_node_id) {
                                    if states.values.len() == 0 || states.values.contains(&origin_state) {
                                        neighbors.push(edge.origin_node_id as i32);
                                    }
                                }
                            }
                        }
                        for (k,v) in graph.edges.iter() {
                            if v.contains(&Edge::new(node_id as usize,0.0)) {
                                if let Some(successor) = graph.nodes.get(k) {
                                    if states.values.len() == 0 || states.values.contains(&successor.get_state()) {
                                        neighbors.push(*k as i32);
                                    }
                                }
                            }
                        }
                        push_state
                            .int_vector_stack
                            .push(IntVector::new(neighbors));
                    }
                }
            }
        }
    }

    /// GRAPH.NODE*PREDECESSORS: Pushes the IDs of the predecessor nodes that are in
    /// one of the predefined states to the INTVECTOR stack. The states are taken from the top 
    /// item of the INTVECTOR stack. If the array is empty all predecessor node IDs are pushed. 
    /// The origin node id is taken from the INTEGER stack.
    fn graph_node_predecessors(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get(0) {
            if let Some(states) = push_state.int_vector_stack.pop() {
                if let Some(node_id) = push_state.int_stack.pop() {
                    if node_id > 0 {
                        let mut predecessors = vec![];
                        if let Some(incoming_edges) = graph.edges.get(&(node_id as usize)) {
                            for edge in incoming_edges {
                                if let Some(origin_state) = graph.get_state(&edge.origin_node_id) {
                                    if states.values.len() == 0 || states.values.contains(&origin_state) {
                                        predecessors.push(edge.origin_node_id as i32);
                                    }
                                }
                            }
                        }
                        push_state
                            .int_vector_stack
                            .push(IntVector::new(predecessors));
                    }
                }
            }
        }
    }

    /// GRAPH.NODE*SUCCESSORS: Pushes the IDs of the successor nodes that are in
    /// one of the predefined states to the INTVECTOR stack. The states are taken from the top 
    /// item of the INTVECTOR stack. If the array is empty all successor node IDs are pushed. 
    /// The origin node id is taken from the INTEGER stack.
    fn graph_node_successors(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(graph) = push_state.graph_stack.get(0) {
            if let Some(states) = push_state.int_vector_stack.pop() {
                if let Some(node_id) = push_state.int_stack.pop() {
                    if node_id > 0 {
                        let mut successors = vec![];
                        for (k,v) in graph.edges.iter() {
                            println!("Checking incoming nodes: {:?}", v);
                            if v.contains(&Edge::new(node_id as usize,0.0)) {
                                if let Some(successor) = graph.nodes.get(k) {
                                    println!("...Found");
                                    if states.values.len() == 0 || states.values.contains(&successor.get_state()) {
                                        successors.push(*k as i32);
                                    }
                                }
                            }
                        }
                        push_state
                            .int_vector_stack
                            .push(IntVector::new(successors));
                    }
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

    /// GRAPH.EDGE*HISTORY: Gets the weight for the edge with the specified stack postition, 
    /// origin and destination id. The stack position is top item of the INTEGER stack
    /// destination and origin ids are second and third items respectively.
    fn graph_edge_history(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
        if let Some(pos) = push_state.int_stack.pop() {
            if pos > 0 {
                 if let Some(graph) = push_state.graph_stack.get_mut(pos as usize) {
                     if let Some(ids) = push_state.int_stack.pop_vec(2) {
                        let origin_id = ids[0] as usize;
                        let destination_id = ids[1] as usize;
                        println!("Origin = {}, Destination = {}", origin_id,destination_id);
                        if let Some(weight) = graph.get_weight(&origin_id, &destination_id) {
                           push_state.float_stack.push(weight);
                        }
                     }
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
    use crate::push::vector::BoolVector;
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
    fn graph_node_selected_predecessors_states_are_pushed() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let predecessor_target_state = 11;
        let predecessor_target_state2 = 12;
        let uninteresting_state = 22;
        let uninteresting_state_2 = 33;
        let destination_state = 44;
        let origin_id1 = test_node(&mut test_state, predecessor_target_state);
        let origin_id2 = test_node(&mut test_state, uninteresting_state);
        let origin_id3 = test_node(&mut test_state, uninteresting_state_2);
        let origin_id4 = test_node(&mut test_state, predecessor_target_state);
        let origin_id5 = test_node(&mut test_state, uninteresting_state);
        let origin_id6 = test_node(&mut test_state, predecessor_target_state2);
        let destination_id = test_node(&mut test_state, destination_state);
        test_edge(&mut test_state, origin_id1, destination_id, 0.1);
        test_edge(&mut test_state, origin_id2, destination_id, 0.1);
        test_edge(&mut test_state, origin_id3, destination_id, 0.1);
        test_edge(&mut test_state, origin_id4, destination_id, 0.1);
        test_edge(&mut test_state, origin_id5, destination_id, 0.1);
        test_edge(&mut test_state, origin_id6, destination_id, 0.1);
        test_state.int_stack.push(destination_id);
        test_state.int_vector_stack.push(IntVector::new(vec![predecessor_target_state, predecessor_target_state2]));
        graph_node_predecessors(&mut test_state, &icache());
        let predecessors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(predecessors.len(), 3);
        assert!(predecessors.contains(&origin_id1));
        assert!(predecessors.contains(&origin_id4));
        assert!(predecessors.contains(&origin_id6));
    }

    #[test]
    fn graph_node_all_predecessors_are_pushed_when_intvector_empty() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let origin_id = test_node(&mut test_state, 1);
        let origin_id2 = test_node(&mut test_state, 1);
        let destination_id = test_node(&mut test_state, 1);
        test_edge(&mut test_state, origin_id, destination_id, 0.1);
        test_edge(&mut test_state, origin_id2, destination_id, 0.1);
        test_state.int_stack.push(destination_id);
        test_state.int_vector_stack.push(IntVector::new(vec![]));
        graph_node_predecessors(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        let predecessors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(predecessors.len(), 2);
        assert!(predecessors.contains(&origin_id));
        assert!(predecessors.contains(&origin_id2));
    }

    #[test]
    fn graph_node_selected_successors_states_are_pushed() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let successor_target_state = 11;
        let successor_target_state2 = 12;
        let uninteresting_state = 22;
        let uninteresting_state_2 = 33;
        let origin_state = 44;
        let destination_id1 = test_node(&mut test_state, successor_target_state);
        let destination_id2 = test_node(&mut test_state, uninteresting_state);
        let destination_id3 = test_node(&mut test_state, uninteresting_state_2);
        let destination_id4 = test_node(&mut test_state, successor_target_state);
        let destination_id5 = test_node(&mut test_state, uninteresting_state);
        let destination_id6 = test_node(&mut test_state, successor_target_state2);
        let origin_id = test_node(&mut test_state, origin_state);
        test_edge(&mut test_state, origin_id, destination_id1, 0.1);
        test_edge(&mut test_state, origin_id, destination_id2, 0.1);
        test_edge(&mut test_state, origin_id, destination_id3, 0.1);
        test_edge(&mut test_state, origin_id, destination_id4, 0.1);
        test_edge(&mut test_state, origin_id, destination_id5, 0.1);
        test_edge(&mut test_state, origin_id, destination_id6, 0.1);
        test_state.int_stack.push(origin_id);
        test_state.int_vector_stack.push(IntVector::new(vec![successor_target_state, successor_target_state2]));
        graph_node_successors(&mut test_state, &icache());
        let successors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(successors.len(), 3);
        assert!(successors.contains(&destination_id1));
        assert!(successors.contains(&destination_id4));
        assert!(successors.contains(&destination_id6));
    }

    #[test]
    fn graph_node_all_successors_are_pushed_when_intvector_empty() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let test_id = test_node(&mut test_state, 1);
        let destination_id1 = test_node(&mut test_state, 1);
        let destination_id2 = test_node(&mut test_state, 1);
        test_edge(&mut test_state, test_id, destination_id1, 0.1);
        test_edge(&mut test_state, test_id, destination_id2, 0.1);
        test_state.int_stack.push(test_id);
        test_state.int_vector_stack.push(IntVector::new(vec![]));
        graph_node_successors(&mut test_state, &icache());
        println!("Graph = {}", test_state.graph_stack.copy(0).unwrap());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        let successors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(successors.len(), 2);
        assert!(successors.contains(&destination_id1));
        assert!(successors.contains(&destination_id2));
    }

    #[test]
    fn graph_node_selected_neighbors_states_are_pushed() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let successor_target_state = 11;
        let successor_target_state2 = 12;
        let predecessor_target_state = 13;
        let predecessor_target_state2 = 14;
        let uninteresting_state = 22;
        let uninteresting_state_2 = 33;
        let origin_state = 44;
        let destination_id1 = test_node(&mut test_state, successor_target_state);
        let destination_id2 = test_node(&mut test_state, uninteresting_state);
        let destination_id3 = test_node(&mut test_state, uninteresting_state_2);
        let destination_id4 = test_node(&mut test_state, successor_target_state);
        let destination_id5 = test_node(&mut test_state, uninteresting_state);
        let destination_id6 = test_node(&mut test_state, successor_target_state2);
        let origin_id1 = test_node(&mut test_state, predecessor_target_state);
        let origin_id2 = test_node(&mut test_state, uninteresting_state);
        let origin_id3 = test_node(&mut test_state, predecessor_target_state2);
        let test_id = test_node(&mut test_state, origin_state);
        test_edge(&mut test_state, test_id, destination_id1, 0.1);
        test_edge(&mut test_state, test_id, destination_id2, 0.1);
        test_edge(&mut test_state, test_id, destination_id3, 0.1);
        test_edge(&mut test_state, test_id, destination_id4, 0.1);
        test_edge(&mut test_state, test_id, destination_id5, 0.1);
        test_edge(&mut test_state, test_id, destination_id6, 0.1);
        test_edge(&mut test_state, origin_id1, test_id, 0.1);
        test_edge(&mut test_state, origin_id2, test_id, 0.1);
        test_edge(&mut test_state, origin_id3, test_id, 0.1);
        test_state.int_stack.push(test_id);
        test_state.int_vector_stack.push(IntVector::new(vec![successor_target_state, successor_target_state2, predecessor_target_state, predecessor_target_state2]));
        graph_node_neighbors(&mut test_state, &icache());
        let neighbors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(neighbors.len(), 5);
        assert!(neighbors.contains(&destination_id1));
        assert!(neighbors.contains(&destination_id4));
        assert!(neighbors.contains(&destination_id6));
        assert!(neighbors.contains(&origin_id1));
        assert!(neighbors.contains(&origin_id3));
    }

    #[test]
    fn graph_node_all_neighbors_are_pushed_when_intvector_empty() {
        let mut test_state = PushState::new();
        graph_add(&mut test_state, &icache());
        let test_id = test_node(&mut test_state, 1);
        let destination_id1 = test_node(&mut test_state, 1);
        let destination_id2 = test_node(&mut test_state, 1);
        let origin_id1 = test_node(&mut test_state, 1);
        let origin_id2 = test_node(&mut test_state, 1);
        test_edge(&mut test_state, test_id, destination_id1, 0.1);
        test_edge(&mut test_state, test_id, destination_id2, 0.1);
        test_edge(&mut test_state, origin_id1, test_id, 0.1);
        test_edge(&mut test_state, origin_id2, test_id, 0.1);
        test_state.int_stack.push(test_id);
        test_state.int_vector_stack.push(IntVector::new(vec![]));
        graph_node_neighbors(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        let neighbors = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&origin_id1));
        assert!(neighbors.contains(&origin_id2));
        assert!(neighbors.contains(&destination_id1));
        assert!(neighbors.contains(&destination_id2));
    }

    #[test]
    fn graph_node_state_modification() {
        let mut test_state = PushState::new();
        let node_state_1 = 94;
        let node_state_2 = 123;
        graph_add(&mut test_state, &icache());
        let node_id = test_node(&mut test_state, node_state_1);
        test_state.int_stack.push(node_id.clone() as i32);
        graph_node_get_state(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), node_state_1);
        test_state.int_stack.push(node_id.clone() as i32);
        test_state.int_stack.push(node_state_2);
        graph_node_set_state(&mut test_state, &icache());
        assert_eq!(
            test_state
                .graph_stack
                .get(0)
                .unwrap()
                .get_state(&(node_id as usize))
                .unwrap(),
          node_state_2
        );
    }

    #[test]
    fn graph_nodes_pushes_selected_ids() {
        let mut test_state = PushState::new();
        let mut test_graph = Graph::new();
        let mut expected_ids = vec![];
        let filter_states = vec![3,4];
        test_graph.add_node(1);
        test_graph.add_node(1);
        test_graph.add_node(1);
        test_graph.add_node(2);
        expected_ids.push(test_graph.add_node(filter_states[0]) as i32);
        expected_ids.push(test_graph.add_node(filter_states[0]) as i32);
        expected_ids.push(test_graph.add_node(filter_states[1]) as i32);
        test_graph.add_node(6);
        test_state.graph_stack.push(test_graph);
        for i in 0..3 {
            test_state.int_stack.push(expected_ids[i].clone());
            test_state.int_stack.push(1);
        }
        graph_node_set_state(&mut test_state, &icache());
        test_state.int_vector_stack.push(IntVector::new(filter_states));
        graph_nodes(&mut test_state, &icache());
        let mut filtered_nodes = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(expected_ids.sort(), filtered_nodes.sort());
    }

    #[test]
    fn graph_nodes_pushes_all_ids_when_filter_is_empty() {
        let mut test_state = PushState::new();
        let mut test_graph = Graph::new();
        let mut expected_ids = vec![];
        expected_ids.push(test_graph.add_node(1) as i32);
        expected_ids.push(test_graph.add_node(112) as i32);
        expected_ids.push(test_graph.add_node(99) as i32);
        expected_ids.push(test_graph.add_node(99) as i32);
        test_state.graph_stack.push(test_graph);
        test_state.int_vector_stack.push(IntVector::new(vec![]));
        graph_nodes(&mut test_state, &icache());
        let mut filtered_nodes = test_state.int_vector_stack.pop().unwrap().values;
        assert_eq!(expected_ids.sort(), filtered_nodes.sort());
    }

    #[test]
    fn graph_node_state_switch_with_unequal_length() {
        let mut test_state = PushState::new();
        let mut test_graph = Graph::new();
        let mut ids_to_switch = vec![];
        let mut state_switch = vec![true; 3];
        state_switch[1] = false;
        let initial_state = 0;
        let on_state = 1;
        let off_state = 2;
        ids_to_switch.push(test_graph.add_node(initial_state) as i32);
        ids_to_switch.push(test_graph.add_node(initial_state) as i32);
        ids_to_switch.push(test_graph.add_node(initial_state) as i32);
        ids_to_switch.push(test_graph.add_node(initial_state) as i32);
        test_state.int_stack.push(on_state);
        test_state.int_stack.push(off_state);
        test_state.int_vector_stack.push(IntVector::new(ids_to_switch.clone()));
        test_state.bool_vector_stack.push(BoolVector::new(state_switch));
        test_state.graph_stack.push(test_graph.clone());
        graph_node_state_switch(&mut test_state, &icache());
        let modified_graph = test_state.graph_stack.pop().unwrap();
        //println!("GRAPH CHANGES = {}", test_graph.diff(&modified_graph).unwrap());
        assert_eq!(modified_graph.get_state(&(ids_to_switch[0] as usize)).unwrap(), on_state); 
        assert_eq!(modified_graph.get_state(&(ids_to_switch[1] as usize)).unwrap(), off_state); 
        assert_eq!(modified_graph.get_state(&(ids_to_switch[2] as usize)).unwrap(), on_state); 
        assert_eq!(modified_graph.get_state(&(ids_to_switch[3] as usize)).unwrap(), initial_state); 
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
        //println!("ograph = {}", test_graph );
        //println!("graph = {}", changed_test_graph );
        println!("test_ids = {:?}", test_ids );
        println!("DIFF = {}", diff );
        assert!(diff.contains("NODES(2)"));
        assert!(diff.contains(&format!("~N[ID: {}, 2 <= STATE => 99]", test_ids[1])));
        assert!(diff.contains(&format!("+N[ID: {}, STATE: 5]", test_ids[4])));
        assert!(diff.contains("EDGES(2)"));
        assert!(diff.contains(&format!("+E[{} <= [ONID: {}, WEIGHT: 1.2]]", test_ids[0], test_ids[4])));
        assert!(diff.contains(&format!("~E[{} <= [ONID: {}, 1.3 <= WEIGHT => 0.2]]",test_ids[0], test_ids[1])));

    }

    #[test]
    fn graph_edge_history_pushes_weight_of_stack_position() {
        let mut test_state = PushState::new();
        let mut test_graph = Graph::new();
        let mut test_ids = vec![];
        let mut test_weights = vec![1.0,2.0,3.0];
        test_ids.push(test_graph.add_node(1));
        test_ids.push(test_graph.add_node(2));
        test_ids.push(test_graph.add_node(3));
        test_ids.push(test_graph.add_node(4));
       
        test_graph.add_edge(test_ids[1], test_ids[0], test_weights[0]);
        test_graph.add_edge(test_ids[2], test_ids[0], test_weights[1]);
        test_graph.add_edge(test_ids[3], test_ids[0], test_weights[2]);
        test_state.graph_stack.push(test_graph.clone());
        
        for _i in 0..3 {
            graph_dup(&mut test_state, &icache());
     
            // Adjust test weights
            test_weights = test_weights.into_iter().map(|x| x + 10.0 ).collect();
            let edit_graph = test_state.graph_stack.get_mut(0).unwrap();

            edit_graph.set_weight(&test_ids[1], &test_ids[0], test_weights[0]);
            edit_graph.set_weight(&test_ids[2], &test_ids[0], test_weights[1]);
            edit_graph.set_weight(&test_ids[3], &test_ids[0], test_weights[2]);
        }

        // Stack position 2
        test_state.int_stack.push(test_ids[1] as i32);
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(1);
        graph_edge_history(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 21.0);
        test_state.int_stack.push(test_ids[2] as i32);
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(1);
        graph_edge_history(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 22.0);
        test_state.int_stack.push(test_ids[3] as i32);
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(1);
        graph_edge_history(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 23.0);

        // Stack position 4
        test_state.int_stack.push(test_ids[1] as i32);
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(3);
        graph_edge_history(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 1.0);
        test_state.int_stack.push(test_ids[2] as i32);
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(3);
        graph_edge_history(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 2.0);
        test_state.int_stack.push(test_ids[3] as i32);
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(3);
        graph_edge_history(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 3.0);
    }

    #[test]
    fn graph_node_history_pushes_state_of_stack_position() {
        let mut test_state = PushState::new();
        let mut test_graph = Graph::new();
        let mut test_ids : Vec<usize> = vec![];
        let mut test_states = vec![1,2];
        test_ids.push(test_graph.add_node(test_states[0]));
        test_ids.push(test_graph.add_node(test_states[1]));
       
        test_state.graph_stack.push(test_graph);
        
        for _i in 0..3 {
            graph_dup(&mut test_state, &icache());
     
            // Adjust test weights
            test_states = test_states.into_iter().map(|x| x + 10 ).collect();
            let edit_graph = test_state.graph_stack.get_mut(0).unwrap();

            edit_graph.set_state(&test_ids[0], test_states[0]);
            edit_graph.set_state(&test_ids[1], test_states[1]);
        }

        // Stack position 2
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(1);
        graph_node_history(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(),21);
        test_state.int_stack.push(test_ids[1] as i32);
        test_state.int_stack.push(1);
        graph_node_history(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(),22);

        // Stack position 4
        test_state.int_stack.push(test_ids[0] as i32);
        test_state.int_stack.push(3);
        graph_node_history(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(),1);
        test_state.int_stack.push(test_ids[1] as i32);
        test_state.int_stack.push(3);
        graph_node_history(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(),2);
    }


}
