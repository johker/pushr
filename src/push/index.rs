use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::state::PushState;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Index {
    pub current: usize,
    pub destination: usize,
}

impl Index {
    pub fn new(dest_arg: usize) -> Self {
        Self {
            current: 0,
            destination: dest_arg,
        }
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.current, self.destination)
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.current == other.current && self.destination == other.destination
    }
}

/// Integer numbers (that is, numbers without decimal points).
pub fn load_index_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(
        String::from("INDEX.CURRENT"),
        Instruction::new(index_current),
    );
    map.insert(String::from("INDEX.DEFINE"), Instruction::new(index_define));
    map.insert(
        String::from("INDEX.DESTINATION"),
        Instruction::new(index_destination),
    );
    map.insert(String::from("INDEX.FLUSH"), Instruction::new(index_flush));
    map.insert(
        String::from("INDEX.INCREASE"),
        Instruction::new(index_increase),
    );
    map.insert(String::from("INDEX.POP"), Instruction::new(index_pop));
}

/// INDEX.CURRENT: Pushes the current field of the top INDEX to the INTEGER stack.
pub fn index_current(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.index_stack.copy(0) {
        push_state.int_stack.push(index.current as i32);
    }
}

/// INDEX.DEFINE: Pushes the top INTEGER as destination of a new index.
pub fn index_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(0, index);
        push_state.index_stack.push(Index::new(corr_index as usize));
    }
}

/// INDEX.DESTINATION: Pushes the destination field of the top INDEX to the INTEGER stack.
pub fn index_destination(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.index_stack.copy(0) {
        push_state
            .index_stack
            .push(Index::new(index.destination as usize));
    }
}

/// INDEX.FLUSH: Flushes the INDEX stack.
pub fn index_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.index_stack.flush();
}

/// INDEX.INCREASE: Increases the current value by one if current < destination. Otherwise
/// this instruction acts as a NOOP.
pub fn index_increase(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.index_stack.get_mut(0) {
        if index.current < index.destination {
            index.current += 1;
        }
    }
}

/// INDEX.POP: Pops the INDEX stack.
pub fn index_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.index_stack.pop();
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn integer_modulus_pushes_result() {
        let mut test_state = PushState::new();
        test_state.index_stack.push(Index::new(3));
        index_increase(&mut test_state, &icache());
        let mut test_index = Index::new(3);
        test_index.current += 1;
        assert_eq!(test_state.index_stack.pop().unwrap(), test_index);
    }
}
