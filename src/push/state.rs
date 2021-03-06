use crate::push::configuration::PushConfiguration;
use crate::push::graph::Graph;
use crate::push::index::Index;
use crate::push::item::Item;
use crate::push::stack::PushStack;
use crate::push::buffer::{PushBuffer, BufferType};
use crate::push::io::{PushMessage};
use crate::push::vector::{BoolVector, FloatVector, IntVector};
use std::collections::HashMap;
use std::fmt;

pub const BOOL_STACK_ID: i32 = 1;
pub const BOOL_VECTOR_STACK_ID: i32 = 2;
pub const CODE_STACK_ID: i32 = 3;
pub const EXEC_STACK_ID: i32 = 4;
pub const FLOAT_STACK_ID: i32 = 5;
pub const FLOAT_VECTOR_STACK_ID: i32 = 6;
pub const INDEX_STACK_ID: i32 = 7;
pub const INPUT_STACK_ID: i32 = 8;
pub const INT_STACK_ID: i32 = 9;
pub const INT_VECTOR_STACK_ID: i32 = 10;
pub const NAME_STACK_ID: i32 = 11;
pub const OUTPUT_STACK_ID: i32 = 12;


pub const INPUT_BUFFER_SIZE: usize = 10;
pub const OUTPUT_BUFFER_SIZE: usize = 3;
pub const GRAPH_BUFFER_SIZE: usize = 100;


pub struct PushState {
    // Scalar Types
    pub bool_stack: PushStack<bool>,
    pub code_stack: PushStack<Item>, // Change this to reference for speedup
    pub exec_stack: PushStack<Item>,
    pub float_stack: PushStack<f32>,
    pub index_stack: PushStack<Index>,
    pub int_stack: PushStack<i32>,
    pub name_stack: PushStack<String>,

    // Vector Types
    pub bool_vector_stack: PushStack<BoolVector>,
    pub float_vector_stack: PushStack<FloatVector>,
    pub int_vector_stack: PushStack<IntVector>,

    // IO
    pub input_stack: PushBuffer<PushMessage>,
    pub output_stack: PushBuffer<PushMessage>,

    // Graph
    pub graph_stack: PushBuffer<Graph>,

    // Bindings
    pub name_bindings: HashMap<String, Item>,

    pub configuration: PushConfiguration,
    pub quote_name: bool,
    pub send_name: bool,
}

impl PushState {
    pub fn new() -> Self {
        Self {
            bool_stack: PushStack::new(),
            code_stack: PushStack::new(),
            exec_stack: PushStack::new(),
            float_stack: PushStack::new(),
            index_stack: PushStack::new(),
            int_stack: PushStack::new(),
            name_stack: PushStack::new(),
            bool_vector_stack: PushStack::new(),
            float_vector_stack: PushStack::new(),
            int_vector_stack: PushStack::new(),
            input_stack: PushBuffer::new(BufferType::Queue, INPUT_BUFFER_SIZE),
            output_stack: PushBuffer::new(BufferType::Queue, OUTPUT_BUFFER_SIZE),
            graph_stack: PushBuffer::new(BufferType::Stack, GRAPH_BUFFER_SIZE),
            name_bindings: HashMap::new(),
            configuration: PushConfiguration::new(),
            quote_name: false,
            send_name: false,
        }
    }

    /// Returns total size of stacks without IO stacks.
    pub fn size(&self) -> usize {
        self.bool_stack.size()
            + self.float_stack.size()
            + self.int_stack.size()
            + self.name_stack.size()
            + self.code_stack.size()
            + self.exec_stack.size()
            + self.bool_vector_stack.size()
            + self.float_vector_stack.size()
            + self.int_vector_stack.size()
    }
}

impl fmt::Display for PushState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut nb = "".to_string();
        let mut sorted: Vec<_> = self.name_bindings.iter().collect();
        sorted.sort_by_key(|a| a.0);

        for (key, value) in &sorted {
            nb += &format!("{} => {}\n ", key, value)[..];
        }
        write!(
            f,
            "> BOOL  : \n{}\n> CODE  : \n{}\n> EXEC  : \n{}\n> FLOAT : \n{}\n> GRAPH : \n{}\n> INDEX : \n{}\n> INT   : \n{}\n> BVEC  : \n{}\n> FVEC  : \n{}\n> IVEC  : \n{}\n> NAME  : \n{}\n> IDS   : \n{}\n",
            self.bool_stack.to_string(),
            self.code_stack.to_string(),
            self.exec_stack.to_string(),
            self.float_stack.to_string(),
            self.graph_stack.to_string(),
            self.index_stack.to_string(),
            self.int_stack.to_string(),
            self.bool_vector_stack.to_string(),
            self.float_vector_stack.to_string(),
            self.int_vector_stack.to_string(),
            self.name_stack.to_string(),
            nb,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_state_prints_name_bindings_in_alphabetical_order() {
        let mut test_state = PushState::new();
        test_state.name_bindings.insert(
            "Var2".to_string(),
            Item::instruction("INTVECTOR.BOOLINDEX".to_string()),
        );
        test_state
            .name_bindings
            .insert("Var1".to_string(), Item::bool(true));
        assert_eq!(test_state.to_string(), "> BOOL  : \n\n> CODE  : \n\n> EXEC  : \n\n> FLOAT : \n\n> GRAPH : \n\n> INDEX : \n\n> INT   : \n\n> BVEC  : \n\n> FVEC  : \n\n> IVEC  : \n\n> NAME  : \n\n> IDS   : \nVar1 => TRUE\n Var2 => INTVECTOR.BOOLINDEX\n \n")
    }
}
