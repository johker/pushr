use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::state::PushState;
use std::collections::HashMap;

/// Floating-point numbers (that is, numbers with decimal points).
pub fn load_float_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("FLOAT.+"), Instruction::new(float_add));
}

/// FLOAT.%: Pushes the second stack item modulo the top stack item. If the top item is zero this
/// acts as a NOOP. The modulus is computed as the remainder of the quotient, where the quotient
/// has first been truncated toward negative infinity.
fn float_modulus(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        if fitems[1] != 0f32 {
            push_state.float_stack.push(fitems[0] % fitems[1]);
        }
    }
}

fn float_mult(push_state: &mut PushState, _instruction_cache: &InstructionCache) {}

fn float_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(pv[0] + pv[1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn float_modulus_pushes() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(-13.4);
        test_state.float_stack.push(1.0);
        float_modulus(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap() + 0.4) < 0.01f32);
    }
}
