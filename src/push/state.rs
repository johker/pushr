use crate::push::configuration::PushConfiguration;
use crate::push::item::Item;
use crate::push::stack::PushStack;
use crate::push::vector::{BoolVector, FloatVector, IntVector};
use std::collections::HashMap;
use std::fmt;

pub struct PushState {
    // Scalar Types
    pub bool_stack: PushStack<bool>,
    pub code_stack: PushStack<Item>,
    pub exec_stack: PushStack<Item>,
    pub float_stack: PushStack<f32>,
    pub int_stack: PushStack<i32>,
    pub name_stack: PushStack<String>,

    // Vector Types
    pub bool_vector_stack: PushStack<BoolVector>,
    pub float_vector_stack: PushStack<FloatVector>,
    pub int_vector_stack: PushStack<IntVector>,

    pub name_bindings: HashMap<String, Item>,
    pub configuration: PushConfiguration,
    pub quote_name: bool,
}

impl PushState {
    pub fn new() -> Self {
        Self {
            float_stack: PushStack::new(),
            exec_stack: PushStack::new(),
            code_stack: PushStack::new(),
            int_stack: PushStack::new(),
            bool_stack: PushStack::new(),
            name_stack: PushStack::new(),
            bool_vector_stack: PushStack::new(),
            float_vector_stack: PushStack::new(),
            int_vector_stack: PushStack::new(),
            name_bindings: HashMap::new(),
            configuration: PushConfiguration::new(),
            quote_name: false,
        }
    }
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
            nb += &format!("{} => {}; ", key, value)[..];
        }
        write!(
            f,
            "> BOOL  : {}\n> CODE  : {}\n> EXEC  : {}\n> FLOAT : {}\n> INT   : {}\n> BVEC  : {}\n>FVEC  : {}\n>IVEC  : {}\n> NAME  : {}\n> IDS   : {}\n",
            self.bool_stack.to_string(),
            self.code_stack.to_string(),
            self.exec_stack.to_string(),
            self.float_stack.to_string(),
            self.int_stack.to_string(),
            self.bool_vector_stack.to_string(),
            self.float_vector_stack.to_string(),
            self.int_vector_stack.to_string(),
            self.name_stack.to_string(),
            nb
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
        assert_eq!(test_state.to_string(), "> BOOL  : \n> CODE  : \n> EXEC  : \n> FLOAT : \n> INT   : \n> BVEC  : \n>FVEC  : \n>IVEC  : \n> NAME  : \n> IDS   : Var1 => Literal(true); Var2 => InstructionMeta(INTVECTOR.BOOLINDEX); \n")
    }
}
