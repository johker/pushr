use crate::prush::instructions::Instruction;
use crate::prush::instructions::InstructionCache;
use crate::prush::item::Item;
use crate::prush::random::CodeGenerator;
use crate::prush::state::PushState;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct BoolVector {
    values: Vec<bool>,
}

impl BoolVector {
    pub fn new(arg: Vec<bool>) -> Self {
        Self { values: arg }
    }
}

impl fmt::Display for BoolVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self
            .values
            .clone()
            .into_iter()
            .fold(String::new(), |acc, num| {
                acc + &(num as u32).to_string() + ","
            });
        s.pop();
        write!(f, "[{}]", s)
    }
}

impl PartialEq for BoolVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[derive(Clone, Debug)]
pub struct IntVector {
    values: Vec<i32>,
}

impl IntVector {
    pub fn new(arg: Vec<i32>) -> Self {
        Self { values: arg }
    }
}

impl fmt::Display for IntVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self
            .values
            .clone()
            .into_iter()
            .fold(String::new(), |acc, num| acc + &num.to_string() + ",");
        s.pop();
        write!(f, "[{}]", s)
    }
}

impl PartialEq for IntVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

#[derive(Clone, Debug)]
pub struct FloatVector {
    values: Vec<f32>,
}

impl FloatVector {
    pub fn new(arg: Vec<f32>) -> Self {
        Self { values: arg }
    }
}

impl fmt::Display for FloatVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = self
            .values
            .clone()
            .into_iter()
            .fold(String::new(), |acc, num| acc + &num.to_string() + ",");
        s.pop();
        write!(f, "[{}]", s)
    }
}

impl PartialEq for FloatVector {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

pub fn load_vector_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(
        String::from("BOOLVECTOR.="),
        Instruction::new(bool_vector_equal),
    );

    map.insert(
        String::from("INTVECTOR.="),
        Instruction::new(int_vector_equal),
    );

    map.insert(
        String::from("FLOATVECTOR.="),
        Instruction::new(float_vector_equal),
    );
}

/////////////////////////////////////// BOOLVECTOR //////////////////////////////////////////

/// BOOLVECTOR.DEFINE: Defines the name on top of the NAME stack as an instruction that will
/// push the top item of the BOOLVECTOR stack onto the EXEC stack.
pub fn bool_vector_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(bvval) = push_state.bool_vector_stack.pop() {
            push_state.name_bindings.insert(name, Item::boolvec(bvval));
        }
    }
}

/// BOOLVECTOR.DUP: Duplicates the top item on the  stack. Does not pop its argument (which, if
/// it did, would negate the effect of the duplication!).
pub fn bool_vector_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bvval) = push_state.bool_vector_stack.copy(0) {
        push_state.bool_vector_stack.push(bvval);
    }
}

/// BOOLVECTOR.=: Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE
/// otherwise.
fn bool_vector_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bvvals) = push_state.bool_vector_stack.pop_vec(2) {
        push_state.bool_stack.push(bvvals[0] == bvvals[1]);
    }
}

/// BOOLVECTOR.FLUSH: Empties the BOOLVECTOR stack.
pub fn bool_vector_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_vector_stack.flush();
}

/// BOOLVECTOR.SHOVE: Inserts the second INTEGER "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn bool_vector_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        push_state.bool_vector_stack.shove(shove_index as usize);
    }
}

/// BOOLVECTOR.STACKDEPTH: Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
pub fn bool_vector_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.bool_vector_stack.size() as i32);
}

/// BOOLVECTOR.SWAP: Swaps the top two BOOLVECTORs.
pub fn bool_vector_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_vector_stack.shove(1);
}

/// BOOLVECTOR.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the
/// stack. The index is taken from the INTEGER stack, and the indexing is done after the index is
/// removed.
pub fn bool_vector_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        push_state.bool_vector_stack.yank(idx as usize);
    }
}

/// BOOLVECTOR.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn bool_vector_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        if let Some(deep_item) = push_state.bool_vector_stack.copy(idx as usize) {
            push_state.bool_vector_stack.push(deep_item);
        }
    }
}
/////////////////////////////////////// INTVECTOR //////////////////////////////////////////

/// INTVECTOR.DEFINE: Defines the name on top of the NAME stack as an instruction that will
/// push the top item of the INTVECTOR stack onto the EXEC stack.
pub fn int_vector_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(ivval) = push_state.int_vector_stack.pop() {
            push_state.name_bindings.insert(name, Item::intvec(ivval));
        }
    }
}

/// INTVECTOR.DUP: Duplicates the top item on the  stack. Does not pop its argument (which, if
/// it did, would negate the effect of the duplication!).
pub fn int_vector_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivval) = push_state.int_vector_stack.copy(0) {
        push_state.int_vector_stack.push(ivval);
    }
}

/// INTVECTOR.=: Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE
/// otherwise.
fn int_vector_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivvals) = push_state.int_vector_stack.pop_vec(2) {
        push_state.bool_stack.push(ivvals[0] == ivvals[1]);
    }
}

/// INTVECTOR.FLUSH: Empties the INTVECTOR stack.
pub fn int_vector_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_vector_stack.flush();
}

/// INTVECTOR.SHOVE: Inserts the second INTEGER "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn int_vector_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        push_state.int_vector_stack.shove(shove_index as usize);
    }
}

/// INTVECTOR.STACKDEPTH: Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
pub fn int_vector_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.int_vector_stack.size() as i32);
}

/// INTVECTOR.SWAP: Swaps the top two INTVECTORs.
pub fn int_vector_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_vector_stack.shove(1);
}

/// INTVECTOR.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the
/// stack. The index is taken from the INTEGER stack, and the indexing is done after the index is
/// removed.
pub fn int_vector_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        push_state.int_vector_stack.yank(idx as usize);
    }
}

/// INTVECTOR.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn int_vector_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        if let Some(deep_item) = push_state.int_vector_stack.copy(idx as usize) {
            push_state.int_vector_stack.push(deep_item);
        }
    }
}

////////////////////////////////////// FLOATVECTOR //////////////////////////////////////////

/// FLOATVECTOR.DEFINE: Defines the name on top of the NAME stack as an instruction that will
/// push the top item of the FLOATVECTOR stack onto the EXEC stack.
pub fn float_vector_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(fvval) = push_state.float_vector_stack.pop() {
            push_state.name_bindings.insert(name, Item::floatvec(fvval));
        }
    }
}

/// FLOATVECTOR.DUP: Duplicates the top item on the  stack. Does not pop its argument (which, if
/// it did, would negate the effect of the duplication!).
pub fn float_vector_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvval) = push_state.float_vector_stack.copy(0) {
        push_state.float_vector_stack.push(fvval);
    }
}

/// FLOATVECTOR.=: Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE
/// otherwise.
fn float_vector_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvvals) = push_state.float_vector_stack.pop_vec(2) {
        push_state.bool_stack.push(fvvals[0] == fvvals[1]);
    }
}

/// FLOATVECTOR.FLUSH: Empties the FLOATVECTOR stack.
pub fn float_vector_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.float_vector_stack.flush();
}

/// FLOATVECTOR.SHOVE: Inserts the second FLOATVECTOR "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn float_vector_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        push_state.float_vector_stack.shove(shove_index as usize);
    }
}

/// FLOATVECTOR.STACKDEPTH: Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
pub fn float_vector_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.float_vector_stack.size() as i32);
}

/// FLOATVECTOR.SWAP: Swaps the top two FLOATVECTORs.
pub fn float_vector_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.float_vector_stack.shove(1);
}

/// FLOATVECTOR.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the
/// stack. The index is taken from the INTEGER stack, and the indexing is done after the index is
/// removed.
pub fn float_vector_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        push_state.float_vector_stack.yank(idx as usize);
    }
}

/// FLOATVECTOR.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn float_vector_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        if let Some(deep_item) = push_state.float_vector_stack.copy(idx as usize) {
            push_state.float_vector_stack.push(deep_item);
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
    fn bool_vector_prints_values() {
        let bv = BoolVector::new(vec![true, false, true]);
        assert_eq!(bv.to_string(), "[1,0,1]");
    }

    #[test]
    fn bool_vector_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true, false]));
        test_state.name_stack.push(String::from("TEST"));
        bool_vector_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::boolvec(BoolVector::new(vec![true, false])).to_string()
        );
    }

    #[test]
    fn bool_vector_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        bool_vector_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn bool_vector_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[1]; 2:[0]; 3:[0]; 4:[0];"
        );
        test_state.int_stack.push(2);
        bool_vector_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[0]; 2:[0]; 3:[1]; 4:[0];"
        );
    }

    #[test]
    fn bool_vector_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        bool_vector_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:4;");
    }

    #[test]
    fn bool_vector_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        assert_eq!(test_state.bool_vector_stack.to_string(), "1:[0]; 2:[1];");
        bool_vector_swap(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.to_string(), "1:[1]; 2:[0];");
    }

    #[test]
    fn bool_vector_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[1]; 2:[1]; 3:[1]; 4:[0]; 5:[1];"
        );
        test_state.int_stack.push(3);
        bool_vector_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[0]; 2:[1]; 3:[1]; 4:[1]; 5:[1];"
        );
    }

    #[test]
    fn bool_vector_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![false]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[1]; 2:[1]; 3:[1]; 4:[0]; 5:[1];"
        );
        test_state.int_stack.push(3);
        bool_vector_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[0]; 2:[1]; 3:[1]; 4:[1]; 5:[0]; 6:[1];"
        );
    }

    #[test]
    fn int_vector_prints_values() {
        let iv = IntVector::new(vec![1, 2, -3]);
        assert_eq!(iv.to_string(), "[1,2,-3]");
    }

    #[test]
    fn int_vector_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(IntVector::new(vec![1, 2]));
        test_state.name_stack.push(String::from("TEST"));
        int_vector_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::intvec(IntVector::new(vec![1, 2])).to_string()
        );
    }

    #[test]
    fn int_vector_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(IntVector::new(vec![4]));
        test_state.int_vector_stack.push(IntVector::new(vec![4]));
        int_vector_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn int_vector_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(IntVector::new(vec![4]));
        test_state.int_vector_stack.push(IntVector::new(vec![3]));
        test_state.int_vector_stack.push(IntVector::new(vec![2]));
        test_state.int_vector_stack.push(IntVector::new(vec![1]));
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[1]; 2:[2]; 3:[3]; 4:[4];"
        );
        test_state.int_stack.push(2);
        int_vector_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[2]; 2:[3]; 3:[1]; 4:[4];"
        );
    }

    #[test]
    fn int_vector_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(IntVector::new(vec![4]));
        test_state.int_vector_stack.push(IntVector::new(vec![3]));
        test_state.int_vector_stack.push(IntVector::new(vec![2]));
        test_state.int_vector_stack.push(IntVector::new(vec![1]));
        int_vector_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:4;");
    }

    #[test]
    fn int_vector_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(IntVector::new(vec![0]));
        test_state.int_vector_stack.push(IntVector::new(vec![1]));
        assert_eq!(test_state.int_vector_stack.to_string(), "1:[1]; 2:[0];");
        int_vector_swap(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.to_string(), "1:[0]; 2:[1];");
    }

    #[test]
    fn int_vector_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(IntVector::new(vec![5]));
        test_state.int_vector_stack.push(IntVector::new(vec![4]));
        test_state.int_vector_stack.push(IntVector::new(vec![3]));
        test_state.int_vector_stack.push(IntVector::new(vec![2]));
        test_state.int_vector_stack.push(IntVector::new(vec![1]));
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[1]; 2:[2]; 3:[3]; 4:[4]; 5:[5];"
        );
        test_state.int_stack.push(3);
        int_vector_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[4]; 2:[1]; 3:[2]; 4:[3]; 5:[5];"
        );
    }

    #[test]
    fn int_vector_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(IntVector::new(vec![5]));
        test_state.int_vector_stack.push(IntVector::new(vec![4]));
        test_state.int_vector_stack.push(IntVector::new(vec![3]));
        test_state.int_vector_stack.push(IntVector::new(vec![2]));
        test_state.int_vector_stack.push(IntVector::new(vec![1]));
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[1]; 2:[2]; 3:[3]; 4:[4]; 5:[5];"
        );
        test_state.int_stack.push(3);
        int_vector_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[4]; 2:[1]; 3:[2]; 4:[3]; 5:[4]; 6:[5];"
        );
    }

    #[test]
    fn float_vector_prints_values() {
        let fv = FloatVector::new(vec![1.2, 3.4, -4.5]);
        assert_eq!(fv.to_string(), "[1.2,3.4,-4.5]");
    }

    #[test]
    fn float_vector_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![1.0, 2.0]));
        test_state.name_stack.push(String::from("TEST"));
        float_vector_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::floatvec(FloatVector::new(vec![1.0, 2.0])).to_string()
        );
    }

    #[test]
    fn float_vector_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![4.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![4.0]));
        float_vector_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn float_vector_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![4.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![3.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![2.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![1.0]));
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[1]; 2:[2]; 3:[3]; 4:[4];"
        );
        test_state.int_stack.push(2);
        float_vector_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[2]; 2:[3]; 3:[1]; 4:[4];"
        );
    }

    #[test]
    fn float_vector_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![4.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![3.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![2.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![1.0]));
        float_vector_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:4;");
    }

    #[test]
    fn float_vector_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![0.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![1.0]));
        assert_eq!(test_state.float_vector_stack.to_string(), "1:[1]; 2:[0];");
        float_vector_swap(&mut test_state, &icache());
        assert_eq!(test_state.float_vector_stack.to_string(), "1:[0]; 2:[1];");
    }

    #[test]
    fn float_vector_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![5.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![4.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![3.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![2.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![1.0]));
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[1]; 2:[2]; 3:[3]; 4:[4]; 5:[5];"
        );
        test_state.int_stack.push(3);
        float_vector_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[4]; 2:[1]; 3:[2]; 4:[3]; 5:[5];"
        );
    }

    #[test]
    fn float_vector_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![5.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![4.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![3.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![2.0]));
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![1.0]));
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[1]; 2:[2]; 3:[3]; 4:[4]; 5:[5];"
        );
        test_state.int_stack.push(3);
        float_vector_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[4]; 2:[1]; 3:[2]; 4:[3]; 5:[4]; 6:[5];"
        );
    }
}
