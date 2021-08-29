use crate::push::state::PushState;
use std::collections::HashMap;

use crate::push::boolean::*;
use crate::push::code::*;
use crate::push::execution::*;
use crate::push::float::*;
use crate::push::integer::*;
use crate::push::io::*;
use crate::push::name::*;
use crate::push::vector::*;

pub struct InstructionSet {
    map: HashMap<String, Instruction>,
}

impl InstructionSet {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Load the default instrcution set for the stack types
    /// bool, int, float, code, exec, name and vector types
    pub fn load(&mut self) {
        self.map
            .insert(String::from("NOOP"), Instruction::new(noop));
        load_boolean_instructions(&mut self.map);
        load_code_instructions(&mut self.map);
        load_exec_instructions(&mut self.map);
        load_float_instructions(&mut self.map);
        load_int_instructions(&mut self.map);
        load_name_instructions(&mut self.map);
        load_vector_instructions(&mut self.map);
        load_io_instructions(&mut self.map);
    }

    /// Create a snapshot of the current instruction names
    pub fn cache(&self) -> InstructionCache {
        InstructionCache::new(self.map.keys().cloned().collect())
    }

    /// Add a new instruction
    pub fn add(&mut self, name: String, instruction: Instruction) -> Option<Instruction> {
        self.map.insert(name, instruction)
    }

    /// Returns true if there exists an instruction
    /// under the given name.
    pub fn is_instruction(&self, name: &str) -> bool {
        match self.map.get(name) {
            Some(_i) => true,
            None => false,
        }
    }

    /// Get a mutable reference of an instruction by name
    pub fn get_instruction(&mut self, name: &str) -> Option<&mut Instruction> {
        self.map.get_mut(name)
    }
}

pub struct InstructionCache {
    pub list: Vec<String>,
}

impl InstructionCache {
    pub fn new(arg_list: Vec<String>) -> Self {
        Self { list: arg_list }
    }
}

pub struct Instruction {
    pub execute: Box<dyn FnMut(&mut PushState, &InstructionCache)>,
}

impl Instruction {
    pub fn new(execute: impl FnMut(&mut PushState, &InstructionCache) + 'static) -> Self {
        Self {
            execute: Box::new(execute),
        }
    }
}

/// NOOP: No operation.
fn noop(_push_state: &mut PushState, _instruction_cache: &InstructionCache) {}
