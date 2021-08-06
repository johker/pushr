use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::random::CodeGenerator;
use crate::push::state::PushState;
use std::collections::HashMap;

/// For creating bindings between symbolic identifiers and values of various types; that is,
/// for implementing (global) variables and defined instructions. Bindings are created with
/// DEFINE instructions. Any identifier that is not a known Push instruction or a known literal
/// of any other type is considered a NAME and will be pushed onto the NAME stack when
/// encountered, unless it has a definition (in which case its associated value will be pushed
/// on the EXEC stack when it is encountered. The NAME.QUOTE instruction can be used to get a
/// name that already has a definition onto the NAME stack.
pub fn load_name_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("NAME.="), Instruction::new(name_equal));
}

/// NAME.=: Pushes TRUE if the top two NAMEs are equal, or FALSE otherwise.
fn name_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(nvals) = push_state.name_stack.pop_vec(2) {
        push_state.bool_stack.push(nvals[0] == nvals[1]);
    }
}

/// NAME.DUP: Duplicates the top item on the NAME stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!).
pub fn name_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(nval) = push_state.name_stack.copy(0) {
        push_state.name_stack.push(nval);
    }
}

/// NAME.FLUSH: Empties the NAME stack.
pub fn name_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.name_stack.flush();
}

/// NAME.POP: Pops the NAME stack.
pub fn name_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.name_stack.pop();
}

/// NAME.QUOTE: Sets a flag indicating that the next name encountered will be pushed onto the NAME
/// stack (and not have its associated value pushed onto the EXEC stack), regardless of whether or
/// not it has a definition. Upon encountering such a name and pushing it onto the NAME stack the
/// flag will be cleared (whether or not the pushed name had a definition).
pub fn name_quote(push_state: &mut PushState, _instruction_cache: &InstructionCache) {}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }
    #[test]
    fn name_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(&"Test");
        test_state.name_stack.push(&"Test");
        name_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn name_dup_copies_top_element() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(&"Test");
        name_dup(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.to_string(), "1:Test; 2:Test;");
    }

    #[test]
    fn name_flush_empties_stack() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(&"I1");
        test_state.name_stack.push(&"I2");
        name_flush(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.to_string(), "");
    }
}
