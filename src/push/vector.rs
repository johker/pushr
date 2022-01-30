use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::random::CodeGenerator;
use crate::push::state::PushState;
use crate::push::state::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct BoolVector {
    pub values: Vec<bool>,
}

impl BoolVector {
    pub fn new(arg: Vec<bool>) -> Self {
        Self { values: arg }
    }

    pub fn from_int_array(arg: Vec<usize>) -> Self {
        let mut bv = vec![false; arg.len()];
        for (i, ival) in arg.iter().enumerate() {
            bv[i] = ival == &1;
        }
        Self { values: bv }
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
    pub values: Vec<i32>,
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
    pub values: Vec<f32>,
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
        String::from("BOOLVECTOR.GET"),
        Instruction::new(bool_vector_get),
    );
    map.insert(
        String::from("BOOLVECTOR.SET"),
        Instruction::new(bool_vector_set),
    );
    map.insert(
        String::from("BOOLVECTOR.AND"),
        Instruction::new(bool_vector_and),
    );
    map.insert(
        String::from("BOOLVECTOR.OR"),
        Instruction::new(bool_vector_or),
    );
    map.insert(
        String::from("BOOLVECTOR.NOT"),
        Instruction::new(bool_vector_not),
    );
    map.insert(
        String::from("BOOLVECTOR.COUNT"),
        Instruction::new(bool_vector_count),
    );
    map.insert(
        String::from("BOOLVECTOR.DEFINE"),
        Instruction::new(bool_vector_define),
    );
    map.insert(
        String::from("BOOLVECTOR.DUP"),
        Instruction::new(bool_vector_dup),
    );
    map.insert(
        String::from("BOOLVECTOR.EQUAL"),
        Instruction::new(bool_vector_equal),
    );
    map.insert(
        String::from("BOOLVECTOR.FLUSH"),
        Instruction::new(bool_vector_flush),
    );
    map.insert(
        String::from("BOOLVECTOR.ID"),
        Instruction::new(bool_vector_id),
    );
    map.insert(
        String::from("BOOLVECTOR.LENGTH"),
        Instruction::new(bool_vector_length),
    );
    map.insert(
        String::from("BOOLVECTOR.ONES"),
        Instruction::new(bool_vector_ones),
    );
    map.insert(
        String::from("BOOLVECTOR.POP"),
        Instruction::new(bool_vector_pop),
    );
    map.insert(
        String::from("BOOLVECTOR.RAND"),
        Instruction::new(bool_vector_rand),
    );
    map.insert(
        String::from("BOOLVECTOR.ROTATE"),
        Instruction::new(bool_vector_rand),
    );
    map.insert(
        String::from("BOOLVECTOR.SHOVE"),
        Instruction::new(bool_vector_shove),
    );
    map.insert(
        String::from("BOOLVECTOR.SORT*ASC"),
        Instruction::new(bool_vector_sort_asc),
    );
    map.insert(
        String::from("BOOLVECTOR.SORT*DESC"),
        Instruction::new(bool_vector_sort_desc),
    );
    map.insert(
        String::from("BOOLVECTOR.SWAP"),
        Instruction::new(bool_vector_swap),
    );
    map.insert(
        String::from("BOOLVECTOR.STACKDEPTH"),
        Instruction::new(bool_vector_stack_depth),
    );
    map.insert(
        String::from("BOOLVECTOR.YANK"),
        Instruction::new(bool_vector_yank),
    );
    map.insert(
        String::from("BOOLVECTOR.YANKDUP"),
        Instruction::new(bool_vector_yank_dup),
    );
    map.insert(
        String::from("BOOLVECTOR.ZEROS"),
        Instruction::new(bool_vector_zeros),
    );

    map.insert(
        String::from("INTVECTOR.APPEND"),
        Instruction::new(int_vector_append),
    );
    map.insert(
        String::from("INTVECTOR.BOOLINDEX"),
        Instruction::new(int_vector_bool_index),
    );
    map.insert(
        String::from("INTVECTOR.GET"),
        Instruction::new(int_vector_get),
    );
    map.insert(
        String::from("INTVECTOR.SET"),
        Instruction::new(int_vector_set),
    );
    map.insert(
        String::from("INTVECTOR.+"),
        Instruction::new(int_vector_add),
    );
    map.insert(
        String::from("INTVECTOR.-"),
        Instruction::new(int_vector_subtract),
    );
//    map.insert(
//        String::from("INTVECTOR.*"),
//        Instruction::new(int_vector_multiply),
//    );
//    map.insert(
//        String::from("INTVECTOR./"),
//        Instruction::new(int_vector_divide),
//    );
    map.insert(
        String::from("INTVECTOR.CONTAINS"),
        Instruction::new(int_vector_contains),
    );
    map.insert(
        String::from("INTVECTOR.DEFINE"),
        Instruction::new(int_vector_define),
    );
    map.insert(
        String::from("INTVECTOR.DUP"),
        Instruction::new(int_vector_dup),
    );
    map.insert(
        String::from("INTVECTOR.EQUAL"),
        Instruction::new(int_vector_equal),
    );
    map.insert(
        String::from("INTVECTOR.FLUSH"),
        Instruction::new(int_vector_flush),
    );
    map.insert(
        String::from("INTVECTOR.FROMINT"),
        Instruction::new(int_vector_from_int),
    );
    map.insert(
        String::from("INTVECTOR.ID"),
        Instruction::new(int_vector_id),
    );
    map.insert(
        String::from("INTVECTOR.ONES"),
        Instruction::new(int_vector_ones),
    );
    map.insert(
        String::from("INTVECTOR.MEAN"),
        Instruction::new(int_vector_mean),
    );
    map.insert(
        String::from("INTVECTOR.LENGTH"),
        Instruction::new(int_vector_length),
    );
    map.insert(
        String::from("INTVECTOR.POP"),
        Instruction::new(int_vector_pop),
    );
    map.insert(
        String::from("INTVECTOR.RAND"),
        Instruction::new(int_vector_rand),
    );
    map.insert(
        String::from("INTVECTOR.ROTATE"),
        Instruction::new(int_vector_rotate),
    );
    map.insert(
        String::from("INTVECTOR.SHOVE"),
        Instruction::new(int_vector_shove),
    );
    map.insert(
        String::from("INTVECTOR.SORT*ASC"),
        Instruction::new(int_vector_sort_asc),
    );
    map.insert(
        String::from("INTVECTOR.SORT*DESC"),
        Instruction::new(int_vector_sort_desc),
    );
    map.insert(
        String::from("INTVECTOR.SWAP"),
        Instruction::new(int_vector_swap),
    );
    map.insert(
        String::from("INTVECTOR.STACKDEPTH"),
        Instruction::new(int_vector_stack_depth),
    );
    map.insert(
        String::from("INTVECTOR.SET*INSERT"),
        Instruction::new(int_vector_set_insert),
    );
    map.insert(
        String::from("INTVECTOR.SUM"),
        Instruction::new(int_vector_sum),
    );
    map.insert(
        String::from("INTVECTOR.YANK"),
        Instruction::new(int_vector_yank),
    );
    map.insert(
        String::from("INTVECTOR.YANKDUP"),
        Instruction::new(int_vector_yank_dup),
    );
    map.insert(
        String::from("INTVECTOR.ZEROS"),
        Instruction::new(int_vector_zeros),
    );

    map.insert(
        String::from("FLOATVECTOR.GET"),
        Instruction::new(float_vector_get),
    );
    map.insert(
        String::from("FLOATVECTOR.SET"),
        Instruction::new(float_vector_set),
    );
    map.insert(
        String::from("FLOATVECTOR.+"),
        Instruction::new(float_vector_add),
    );
    map.insert(
        String::from("FLOATVECTOR.-"),
        Instruction::new(float_vector_subtract),
    );
    map.insert(
        String::from("FLOATVECTOR.*"),
        Instruction::new(float_vector_multiply),
    );
    map.insert(
        String::from("FLOATVECTOR.*SCALAR"),
        Instruction::new(float_vector_multiply_scalar),
    );
    map.insert(
        String::from("FLOATVECTOR./"),
        Instruction::new(float_vector_divide),
    );
    map.insert(
        String::from("FLOATVECTOR.DEFINE"),
        Instruction::new(float_vector_define),
    );
    map.insert(
        String::from("FLOATVECTOR.DUP"),
        Instruction::new(float_vector_dup),
    );
    map.insert(
        String::from("FLOATVECTOR.EQUAL"),
        Instruction::new(float_vector_equal),
    );
    map.insert(
        String::from("FLOATVECTOR.FLUSH"),
        Instruction::new(float_vector_flush),
    );
    map.insert(
        String::from("FLOATVECTOR.ID"),
        Instruction::new(float_vector_id),
    );
    map.insert(
        String::from("FLOATVECTOR.LENGTH"),
        Instruction::new(float_vector_length),
    );
    map.insert(
        String::from("FLOATVECTOR.MEAN"),
        Instruction::new(float_vector_mean),
    );
    map.insert(
        String::from("FLOATVECTOR.ONES"),
        Instruction::new(float_vector_ones),
    );
    map.insert(
        String::from("FLOATVECTOR.POP"),
        Instruction::new(float_vector_pop),
    );
    map.insert(
        String::from("FLOATVECTOR.RAND"),
        Instruction::new(float_vector_rand),
    );
    map.insert(
        String::from("FLOATVECTOR.ROTATE"),
        Instruction::new(float_vector_rotate),
    );
    map.insert(
        String::from("FLOATVECTOR.SINE"),
        Instruction::new(float_vector_sine),
    );
    map.insert(
        String::from("FLOATVECTOR.SHOVE"),
        Instruction::new(float_vector_shove),
    );
    map.insert(
        String::from("FLOATVECTOR.SORT*ASC"),
        Instruction::new(float_vector_sort_asc),
    );
    map.insert(
        String::from("FLOATVECTOR.SORT*DESC"),
        Instruction::new(float_vector_sort_desc),
    );
    map.insert(
        String::from("FLOATVECTOR.SWAP"),
        Instruction::new(float_vector_swap),
    );
    map.insert(
        String::from("FLOATVECTOR.STACKDEPTH"),
        Instruction::new(float_vector_stack_depth),
    );
    map.insert(
        String::from("FLOATVECTOR.SUM"),
        Instruction::new(float_vector_stack_depth),
    );
    map.insert(
        String::from("FLOATVECTOR.YANK"),
        Instruction::new(float_vector_yank),
    );
    map.insert(
        String::from("FLOATVECTOR.YANKDUP"),
        Instruction::new(float_vector_yank_dup),
    );
    map.insert(
        String::from("FLOATVECTOR.ZEROS"),
        Instruction::new(float_vector_zeros),
    );
}

/////////////////////////////////////// BOOLVECTOR //////////////////////////////////////////

/// BOOLVECTOR.ID: Pushes the ID of the BOOLVECTOR stack to the INTEGER stack.
pub fn bool_vector_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(BOOL_VECTOR_STACK_ID);
}

/// BOOLVECTOR.SET: Replaces the ith element of the top BOOLVECTOR item by the top item of the
/// BOOLEAN stack. The index i is taken from the INTEGER stack.
pub fn bool_vector_set(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(new_element) = push_state.bool_stack.pop() {
            if let Some(item_to_change) = push_state.bool_vector_stack.get_mut(0) {
                let i =
                    i32::max(i32::min(index, item_to_change.values.len() as i32 - 1), 0) as usize;
                item_to_change.values[i] = new_element;
            }
        }
    }
}

/// BOOLVECTOR.AND: Pushes the result of applying element-wise AND of the top item to the
/// second item on the BOOLVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn bool_vector_and(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut bv) = push_state.bool_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = bv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                bv[0].values[ofs_idx] &= bv[1].values[i];
            }
            push_state.bool_vector_stack.push(bv[0].clone());
        }
    }
}

/// BOOLVECTOR.GET: Copies the element at index i of the top BOOLVECTOR item to the BOOLEAN stack
/// where i taken from the INTEGER stack limited to valid range.
pub fn bool_vector_get(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(element) = push_state.bool_vector_stack.get(0) {
            let i = i32::max(i32::min(index, element.values.len() as i32 - 1), 0) as usize;
            push_state.bool_stack.push(element.values[i].clone());
        }
    }
}

/// BOOLVECTOR.OR: Pushes the result of applying element-wise OR of the top item to the
/// second item on the BOOLVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn bool_vector_or(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut bv) = push_state.bool_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = bv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                bv[0].values[ofs_idx] |= bv[1].values[i];
            }
            push_state.bool_vector_stack.push(bv[0].clone());
        }
    }
}

/// BOOLVECTOR.NOT Applies the negation operator for the elements of the top item. It only considers
/// indices larger than the offset. The offset is taken from the INTEGER stack.
pub fn bool_vector_not(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut bvval) = push_state.bool_vector_stack.pop() {
        if let Some(offset) = push_state.int_stack.pop() {
            for i in 0..bvval.values.len() {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > bvval.values.len() - 1 {
                    continue; // Out of bounds
                }
                bvval.values[ofs_idx] = !bvval.values[ofs_idx];
            }
            push_state.bool_vector_stack.push(bvval.clone());
        }
    }
}

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

/// BOOLVECTOR.LENGTH: Pushes the length of the top BOOLVECTOR item to the INTEGER stack.
pub fn bool_vector_length(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bv) = push_state.bool_vector_stack.get(0) {
        push_state.int_stack.push(bv.values.len() as i32);
    }
}

/// BOOLVECTOR.ONES: Pushes a newly generated BOOLVECTOR with all elements set to true. The size
/// is taken from the INTEGER stack
pub fn bool_vector_ones(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if size > 0 {
            push_state
                .bool_vector_stack
                .push(BoolVector::from_int_array(vec![1; size as usize]));
        }
    }
}

/// BOOLVECTOR.POP: Pops the BOOLVECTOR stack.
pub fn bool_vector_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_vector_stack.pop();
}

/// BOOLVECTOR.RAND: Pushes a newly generated random BOOLVECTOR. The size is taken from the INTEGER
/// stack, the sparsity from the FLOAT stack. If the size is <0 or the sparcity not in [0,1] this
/// acts as a NOOP.
pub fn bool_vector_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if let Some(sparsity) = push_state.float_stack.pop() {
            if let Some(rbvval) = CodeGenerator::random_bool_vector(size, sparsity) {
                push_state.bool_vector_stack.push(rbvval);
            }
        }
    }
}

/// BOOLVECTOR.ROTATE: Moves all elements of the top item to the adjacent position on the left.
/// The first item is removed while the last element of the vector is taken from the BOOLEAN stack.
pub fn bool_vector_rotate(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(b) = push_state.bool_stack.pop() {
        if let Some(bv) = push_state.bool_vector_stack.get_mut(0) {
            bv.values.rotate_left(1);
            let n = bv.values.len();
            bv.values[n - 1] = b;
        }
    }
}

/// BOOLVECTOR.SORT*ASC: Sorts the top BOOLVECTOR item in ascending order.
pub fn bool_vector_sort_asc(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bvec) = push_state.bool_vector_stack.get_mut(0) {
        bvec.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
}

/// BOOLVECTOR.SORT*DESC: Sorts the top BOOLVECTOR item in descending order.
pub fn bool_vector_sort_desc(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bvec) = push_state.bool_vector_stack.get_mut(0) {
        bvec.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        bvec.values.reverse();
    }
}

/// BOOLVECTOR.COUNT Pushes the count of true elements to the INTEGER stack.
pub fn bool_vector_count(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bvec) = push_state.bool_vector_stack.get(0) {
        push_state
            .int_stack
            .push(bvec.values.iter().filter(|&n| *n == true).count() as i32);
    }
}

/// BOOLVECTOR.SHOVE: Inserts the second INTEGER "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn bool_vector_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min(
                (push_state.bool_vector_stack.size() as i32) - 1,
                shove_index,
            ),
            0,
        ) as usize;
        push_state.bool_vector_stack.shove(corr_index);
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
        let corr_index = i32::max(
            i32::min((push_state.bool_vector_stack.size() as i32) - 1, idx),
            0,
        ) as usize;
        push_state.bool_vector_stack.yank(corr_index as usize);
    }
}

/// BOOLVECTOR.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn bool_vector_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.bool_vector_stack.size() as i32) - 1, idx),
            0,
        ) as usize;
        if let Some(deep_item) = push_state.bool_vector_stack.copy(corr_index as usize) {
            push_state.bool_vector_stack.push(deep_item);
        }
    }
}

/// BOOLVECTOR.ZEROS: Pushes a newly generated BOOLVECTOR with all elements set to false. The size
/// is taken from the INTEGER stack.
pub fn bool_vector_zeros(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if size > 0 {
            push_state
                .bool_vector_stack
                .push(BoolVector::from_int_array(vec![0; size as usize]));
        }
    }
}

/////////////////////////////////////// INTVECTOR //////////////////////////////////////////

/// INTVECTOR.APPEND: Appends the top integer item to the top intvector item.
pub fn int_vector_append(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    if let Some(item) = push_state.int_vector_stack.get_mut(0) {
        if let Some(to_append) = push_state.int_stack.pop() {
            item.values.push(to_append);
        }
    }
}

/// INTVECTOR.SET*INSERT: Appends the top integer item to the top intvector item - only if 
/// it does not already exit in the intvector.
pub fn int_vector_set_insert(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    if let Some(item) = push_state.int_vector_stack.get_mut(0) {
        if let Some(to_insert) = push_state.int_stack.pop() {
            if !item.values.contains(&to_insert) {
                item.values.push(to_insert);
            }
        }
    }
}

/// INTVECTOR.ID: Pushes the ID of the INTVECTOR stack to the INTEGER stack.
pub fn int_vector_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(INT_VECTOR_STACK_ID);
}

/// INTVECTOR.BOOLINDEX: Pushes an INTVECTOR item that contains the indices of all true values
/// of the top BOOLVECTOR item. For example, this instruction pushes INT[0,2] if the top
/// item on the BOOLVECTOR stack is BOOL[1,0,1]. The BOOLVECTOR item is popped.
pub fn int_vector_bool_index(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(bvval) = push_state.bool_vector_stack.pop() {
        let mut index_vector = vec![];
        for (i, bval) in bvval.values.iter().enumerate() {
            if *bval {
                index_vector.push(i as i32);
            }
        }
        push_state
            .int_vector_stack
            .push(IntVector::new(index_vector));
    }
}

/// INTVECTOR.GET: Copies the element at index i of the top INTVECTOR item to the INTEGER stack
/// where i taken from the INTEGER stack and bound to valid range.
pub fn int_vector_get(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(element) = push_state.int_vector_stack.get(0) {
            let i = i32::max(i32::min(index, element.values.len() as i32 - 1), 0) as usize;
            if i < element.values.len() {
                push_state.int_stack.push(element.values[i].clone());
            }
        }
    }
}

/// INTVECTOR.SET: Replaces the ith element of the top INTVECTOR item by the second item of the
/// INTVECTOR stack. The top item of the INTEGER stack is the index i bound to valid range.
pub fn int_vector_set(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(new_element) = push_state.int_stack.pop() {
            if let Some(item_to_change) = push_state.int_vector_stack.get_mut(0) {
                let i =
                    i32::max(i32::min(index, item_to_change.values.len() as i32 - 1), 0) as usize;
                item_to_change.values[i] = new_element;
            }
        }
    }
}

/// INTVECTOR.+: Pushes the result of applying element-wise ADD of the top item to the
/// second item on the INTVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn int_vector_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.int_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                iv[0].values[ofs_idx] += iv[1].values[i];
            }
            push_state.int_vector_stack.push(iv[0].clone());
        }
    }
}

/// INTVECTOR.-: Pushes the result of element-wise SUBTRACT of the top item from the
/// second item on the INTVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn int_vector_subtract(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.int_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                iv[0].values[ofs_idx] -= iv[1].values[i];
            }
            push_state.int_vector_stack.push(iv[0].clone());
        }
    }
}

/// INTVECTOR.*: Pushes the result of element-wise MULTIPLY of the top item to the
/// second item on the INTVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn int_vector_multiply(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.int_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                iv[0].values[ofs_idx] *= iv[1].values[i];
            }
            push_state.int_vector_stack.push(iv[0].clone());
        }
    }
}

/// INTVECTOR./: Pushes the result of element-wise DIVIDE of the second item by the
/// top item on the INTVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result. If at least one divisor is zero the instruction acts
/// as NOOP.
pub fn int_vector_divide(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.int_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            let mut invalid = false;
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                if iv[1].values[i] == 0 {
                    invalid = true;
                } else {
                    iv[0].values[ofs_idx] /= iv[1].values[i];
                }
            }
            if !invalid {
                push_state.int_vector_stack.push(iv[0].clone());
            }
        }
    }
}

/// INTVECTOR.CONTAINS: Pushes true to the BOOLEAN stack if the top INTEGER is included in the
/// top INTVECTOR item. This instruction acts as a NOOP if there is no INTEGER or INTVECTOR.
/// The INTVECTOR items is not popped.
pub fn int_vector_contains(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(element) = push_state.int_stack.pop() {
        if let Some(array) = push_state.int_vector_stack.get(0) {
            push_state.bool_stack.push(array.values.contains(&element));
        }
    }
}

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

/// INTVECTOR.FROMINT: Create an INTVECTOR from the elements of the INTEGER stack. The top
/// element 0 (min-max corrected) describes the number of elements. The elements 1..n of
/// the INTEGER stack are pushed as vector to the INTVECTOR stack.
pub fn int_vector_from_int(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(vector_size) = push_state.int_stack.pop() {
        let size = push_state.int_stack.size() as i32;
        let corr_size = i32::max(i32::min(size, vector_size), 0) as usize;
        if let Some(ivec) = push_state.int_stack.pop_vec(corr_size) {
            push_state.int_vector_stack.push(IntVector::new(ivec));
        }
    }
}

/// INTVECTOR.LENGTH: Pushes the length of the top INTVECTOR item to the INTEGER stack.
pub fn int_vector_length(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(iv) = push_state.int_vector_stack.get(0) {
        push_state.int_stack.push(iv.values.len() as i32);
    }
}

/// INTVECTOR.MEAN: Pushes the mean of the top INTVECTOR to the float stack
pub fn int_vector_mean(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(numbers) = push_state.int_vector_stack.get(0) {
        let sum = numbers.values.iter().sum::<i32>() as f32;
        let size = numbers.values.len() as f32;
        push_state.float_stack.push(sum / size);
    }
}

/// INTVECTOR.ONES: Pushes a newly generated INTVECTOR with all elements set to 1. The size
/// is taken from the INTEGER stack
pub fn int_vector_ones(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if size > 0 {
            push_state
                .int_vector_stack
                .push(IntVector::new(vec![1; size as usize]));
        }
    }
}

/// INTVECTOR.POP: Pops the INTVECTOR stack.
pub fn int_vector_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_vector_stack.pop();
}

/// INTVECTOR.RAND: Pushes a newly generated random INTVECTOR. The size, min and max values
/// taken from the INTEGER stack in that order. If the size is <0 or max < min this act as a NOOP.
pub fn int_vector_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(params) = push_state.int_stack.pop_vec(3) {
        // 1 params[2] -> size
        // 2 params[1] -> max
        // 3 params[0] -> min
        if let Some(rbvval) = CodeGenerator::random_int_vector(params[2], params[0], params[1]) {
            push_state.int_vector_stack.push(rbvval);
        }
    }
}

/// INTVECTOR.ROTATE: Moves all elements of the top item to the adjacent position on the left.
/// The first item is removed while the last element of the vector is taken from the INTEGER stack.
pub fn int_vector_rotate(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(i) = push_state.int_stack.pop() {
        if let Some(iv) = push_state.int_vector_stack.get_mut(0) {
            iv.values.rotate_left(1);
            let n = iv.values.len();
            iv.values[n - 1] = i;
        }
    }
}

/// INTVECTOR.SHOVE: Inserts the second INTEGER "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn int_vector_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.int_vector_stack.size() as i32) - 1, shove_index),
            0,
        ) as usize;
        push_state.int_vector_stack.shove(corr_index as usize);
    }
}

/// INTVECTOR.SORT*ASC: Sorts the top INTVECTOR item in ascending order.
pub fn int_vector_sort_asc(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivec) = push_state.int_vector_stack.get_mut(0) {
        ivec.values.sort();
    }
}

/// INTVECTOR.SORT*DESC: Sorts the top INTVECTOR item in descending order.
pub fn int_vector_sort_desc(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivec) = push_state.int_vector_stack.get_mut(0) {
        ivec.values.sort();
        ivec.values.reverse();
    }
}

/// INTVECTOR.STACKDEPTH: Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
pub fn int_vector_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.int_vector_stack.size() as i32);
}

/// INTVECTOR.SUM Pushes the sum of the elements to the INTEGER stack.
pub fn int_vector_sum(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivec) = push_state.int_vector_stack.get(0) {
        push_state.int_stack.push(ivec.values.iter().sum());
    }
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
        let corr_index = i32::max(
            i32::min((push_state.int_vector_stack.size() as i32) - 1, idx),
            0,
        ) as usize;
        push_state.int_vector_stack.yank(corr_index as usize);
    }
}

/// INTVECTOR.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn int_vector_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.int_vector_stack.size() as i32) - 1, idx),
            0,
        ) as usize;
        if let Some(deep_item) = push_state.int_vector_stack.copy(corr_index as usize) {
            push_state.int_vector_stack.push(deep_item);
        }
    }
}

/// INTVECTOR.ZEROS: Pushes a newly generated INTVECTOR with all elements set to 0. The size
/// is taken from the INTEGER stack
pub fn int_vector_zeros(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if size > 0 {
            push_state
                .int_vector_stack
                .push(IntVector::new(vec![0; size as usize]));
        }
    }
}

////////////////////////////////////// FLOATVECTOR //////////////////////////////////////////

/// FLOATVECTOR.ID: Pushes the ID of the FLOATVECTOR stack to the INTEGER stack.
pub fn float_vector_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(FLOAT_VECTOR_STACK_ID);
}

/// FLOATVECTOR.GET: Copies the element at index i of the top FLOATVECTOR item to the FLOAT stack
/// where i is taken from the FLOAT stack limited to valid range.
pub fn float_vector_get(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(element) = push_state.float_vector_stack.get(0) {
            let i = i32::max(i32::min(index, element.values.len() as i32 - 1), 0) as usize;
            push_state.float_stack.push(element.values[i].clone());
        }
    }
}

/// FLOATVECTOR.SET: Replaces the ith element of the top FLOATVECTOR item by the top item of the
/// FLOAT stack. The top item of the INTEGER stack is the index i limited to valid range.
pub fn float_vector_set(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        if let Some(new_element) = push_state.float_stack.pop() {
            if let Some(item_to_change) = push_state.float_vector_stack.get_mut(0) {
                let i =
                    i32::max(i32::min(index, item_to_change.values.len() as i32 - 1), 0) as usize;
                item_to_change.values[i] = new_element;
            }
        }
    }
}

/// FLOATVECTOR.+: Pushes the result of applying element-wise ADD of the top item to the
/// second item on the FLOATVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn float_vector_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.float_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                iv[0].values[ofs_idx] += iv[1].values[i];
            }
            push_state.float_vector_stack.push(iv[0].clone());
        }
    }
}

/// FLOATVECTOR.-: Pushes the result of element-wise SUBTRACT of the top item from the
/// second item on the INTVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn float_vector_subtract(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.float_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                iv[0].values[ofs_idx] -= iv[1].values[i];
            }
            push_state.float_vector_stack.push(iv[0].clone());
        }
    }
}

/// FLOATVECTOR.*: Pushes the result of element-wise MULTIPLY of the top item to the
/// second item on the INTVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result.
pub fn float_vector_multiply(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.float_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                iv[0].values[ofs_idx] *= iv[1].values[i];
            }
            push_state.float_vector_stack.push(iv[0].clone());
        }
    }
}

/// FLOATVECTOR./: Pushes the result of element-wise DIVIDE of the second item by the
/// top item on the FLOATVECTOR stack. It applies an offset to the indices of the top
/// item. The offset is taken from the INTEGER stack. Indices that are outside of the valid
/// range of the second item are ignored. If there is no overlap of indices the second item of
/// the stack is pushed as a result. If at least one divisor is zero the instruction acts
/// as NOOP.
pub fn float_vector_divide(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(mut iv) = push_state.float_vector_stack.pop_vec(2) {
        if let Some(offset) = push_state.int_stack.pop() {
            let mut invalid = false;
            // Loop through indices of second item
            let scd_size = iv[0].values.len();
            for i in 0..scd_size {
                let ofs_idx = (i as i32 + offset) as usize;
                if ofs_idx > scd_size - 1 {
                    continue; // Out of bounds
                }
                if iv[1].values[i] == 0.0 {
                    invalid = true;
                } else {
                    iv[0].values[ofs_idx] /= iv[1].values[i];
                }
            }
            if !invalid {
                push_state.float_vector_stack.push(iv[0].clone());
            }
        }
    }
}

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

/// FLOATVECTOR.LENGTH: Pushes the length of the top FLOATVECTOR item to the INTEGER stack.
pub fn float_vector_length(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fv) = push_state.float_vector_stack.get(0) {
        push_state.int_stack.push(fv.values.len() as i32);
    }
}

/// FLOATVECTOR.MEAN: Pushes the mean of the top FLOATVECTOR to the float stack
pub fn float_vector_mean(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(numbers) = push_state.float_vector_stack.get(0) {
        let sum = numbers.values.iter().sum::<f32>();
        let size = numbers.values.len() as f32;
        push_state.float_stack.push(sum / size);
    }
}

/// FLOATVECTOR.*SCALAR: Multiplies the top item of the FLOAT stack with each element of the
/// top FLOATVECTOR element.
pub fn float_vector_multiply_scalar(
    push_state: &mut PushState,
    _instruction_cache: &InstructionCache,
) {
    if let Some(f) = push_state.float_stack.pop() {
        if let Some(fv) = push_state.float_vector_stack.get_mut(0) {
            fv.values.iter_mut().for_each(|x| *x *= f);
        }
    }
}

/// FLOATVECTOR.ONES: Pushes a newly generated FLOATVECTOR with all elements set to 1. The size
/// is taken from the INTEGER stack
pub fn float_vector_ones(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if size > 0 {
            push_state
                .float_vector_stack
                .push(FloatVector::new(vec![1.0; size as usize]));
        }
    }
}

/// FLOATVECTOR.POP: Pops the FLOATVECTOR stack.
pub fn float_vector_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.float_vector_stack.pop();
}

/// FLOATVECTOR.RAND: Pushes a newly generated random INTVECTOR. The size is taken from the
/// INTEGER stack while the parameters for mean and standard deviation are the first (top) and
/// second item on the FLOAT stack. If size < 0 or standard deviation < 0 this act as a NOOP.
pub fn float_vector_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if let Some(gauss_params) = push_state.float_stack.pop_vec(2) {
            // 1 gauss_params[1]: mean
            // 2 gauss_params[0]: stddev
            if let Some(rfvval) =
                CodeGenerator::random_float_vector(size, gauss_params[1], gauss_params[0])
            {
                push_state.float_vector_stack.push(rfvval);
            }
        }
    }
}

/// FLOATVECTOR.ROTATE: Moves all elements of the top item to the adjacent position on the left.
/// The first item is removed while the last element of the vector is taken from the FLOAT stack.
pub fn float_vector_rotate(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(f) = push_state.float_stack.pop() {
        if let Some(fv) = push_state.float_vector_stack.get_mut(0) {
            fv.values.rotate_left(1);
            let n = fv.values.len();
            fv.values[n - 1] = f;
        }
    }
}

/// FLOATVECTOR.SINE: Pushes a FLOATVECTOR item whose elements describe a sine wave. The sine wave
/// for the element at index i is calulated as A*sin(2*pi*x*i + phi). The amplitude A (1st),
/// the angle velocity x (2nd) and the phase angle phi (3rd) are taken from the FLOAT stack
/// (in that order). The vector length is taken from the INTEGER stack.
pub fn float_vector_sine(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(sine_params) = push_state.float_stack.pop_vec(3) {
        if let Some(vector_size) = push_state.int_stack.pop() {
            let mut sine_vector = vec![];
            for i in 0..vector_size as usize {
                sine_vector.push(
                    sine_params[2]
                        * (2.0 * std::f32::consts::PI * sine_params[1] * i as f32 + sine_params[0])
                            .sin(),
                )
            }
            push_state
                .float_vector_stack
                .push(FloatVector::new(sine_vector));
        }
    }
}

/// FLOATVECTOR.SHOVE: Inserts the second FLOATVECTOR "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn float_vector_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min(
                (push_state.float_vector_stack.size() as i32) - 1,
                shove_index,
            ),
            0,
        ) as usize;
        push_state.float_vector_stack.shove(corr_index as usize);
    }
}

/// FLOATVECTOR.SORT*ASC: Sorts the top FLOATVECTOR item in ascending order.
pub fn float_vector_sort_asc(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvec) = push_state.float_vector_stack.get_mut(0) {
        fvec.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
}

/// FLOATVECTOR.SORT*DESC: Sorts the top FLOATVECTOR item in descending order.
pub fn float_vector_sort_desc(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvec) = push_state.float_vector_stack.get_mut(0) {
        fvec.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        fvec.values.reverse();
    }
}

/// FLOATVECTOR.STACKDEPTH: Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
pub fn float_vector_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.float_vector_stack.size() as i32);
}

/// FLOATVECTOR.SUM Pushes the sum of the elements to the FLOAT stack.
pub fn float_vector_sum(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvec) = push_state.float_vector_stack.get(0) {
        push_state.float_stack.push(fvec.values.iter().sum());
    }
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
        let corr_index = i32::max(
            i32::min((push_state.float_vector_stack.size() as i32) - 1, idx),
            0,
        ) as usize;
        push_state.float_vector_stack.yank(corr_index as usize);
    }
}

/// FLOATVECTOR.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn float_vector_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.float_vector_stack.size() as i32) - 1, idx),
            0,
        ) as usize;
        if let Some(deep_item) = push_state.float_vector_stack.copy(corr_index as usize) {
            push_state.float_vector_stack.push(deep_item);
        }
    }
}

/// FLOATVECTOR.ZEROS: Pushes a newly generated FLOATVECTOR with all elements set to 0. The size
/// is taken from the INTEGER stack
pub fn float_vector_zeros(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(size) = push_state.int_stack.pop() {
        if size > 0 {
            push_state
                .float_vector_stack
                .push(FloatVector::new(vec![0.0; size as usize]));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    /////////////////////////////////////// BOOLVECTOR //////////////////////////////////////////

    #[test]
    fn bool_vector_prints_values() {
        let bv = BoolVector::new(vec![true, false, true]);
        assert_eq!(bv.to_string(), "[1,0,1]");
    }

    #[test]
    fn bool_vector_and_with_different_overlaps() {
        let test_vec1 = BoolVector::from_int_array(vec![1, 1, 1, 1, 0, 0, 0, 0]);
        let test_vec2 = BoolVector::from_int_array(vec![1, 0, 1, 0, 1, 0, 1, 0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec2.clone());
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(0);
        bool_vector_and(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 0, 1, 0, 0, 0, 0, 0])
        );

        // Positive overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec2.clone());
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(-4);
        bool_vector_and(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![0, 0, 0, 0, 1, 0, 1, 0])
        );

        // No overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec2.clone());
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(8);
        bool_vector_and(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 0, 1, 0, 1, 0, 1, 0])
        );
    }

    #[test]
    fn bool_vector_get_pushes_vector_element() {
        let test_vec1 = BoolVector::from_int_array(vec![1, 1, 1, 0, 1, 1, 1, 1]);
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec1);
        test_state.int_stack.push(3);
        bool_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), false);
        // Invalid index is bound to valid range
        test_state.int_stack.push(15);
        bool_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn bool_vector_set_modifies_vector() {
        let test_vec1 = BoolVector::from_int_array(vec![1, 1, 1, 1, 1, 1, 1, 1]);
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec1);
        test_state.int_stack.push(5);
        test_state.bool_stack.push(false);
        bool_vector_set(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 1, 1, 1, 1, 0, 1, 1])
        );
    }

    #[test]
    fn bool_vector_or_with_different_overlaps() {
        let test_vec1 = BoolVector::from_int_array(vec![1, 1, 1, 1, 0, 0, 0, 0]);
        let test_vec2 = BoolVector::from_int_array(vec![1, 0, 1, 0, 1, 0, 1, 0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec2.clone());
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(0);
        bool_vector_or(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 1, 1, 1, 1, 0, 1, 0])
        );

        // Positive overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec2.clone());
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(-4);
        bool_vector_or(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 0, 1, 0, 1, 0, 1, 0])
        );

        // No overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec2.clone());
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(8);
        bool_vector_or(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 0, 1, 0, 1, 0, 1, 0])
        );
    }

    #[test]
    fn bool_vector_not_with_different_overlaps() {
        let test_vec1 = BoolVector::from_int_array(vec![1, 1, 1, 1, 0, 0, 0, 0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(0);
        bool_vector_not(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![0, 0, 0, 0, 1, 1, 1, 1])
        );

        // Positive overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(-4);
        bool_vector_not(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![0, 0, 0, 0, 0, 0, 0, 0])
        );

        // No overlap
        let mut test_state = PushState::new();
        test_state.bool_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(8);
        bool_vector_not(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 1);
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 1, 1, 1, 0, 0, 0, 0])
        );
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
    fn bool_vector_ones_creates_item() {
        let mut test_state = PushState::new();
        let mut test_size = -11;
        test_state.int_stack.push(test_size);
        bool_vector_ones(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 0);
        test_size = 11;
        test_state.int_stack.push(test_size);
        bool_vector_ones(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1; test_size as usize])
        );
    }

    #[test]
    fn bool_vector_rand_pushes_new_item() {
        let mut test_state = PushState::new();
        let test_size = 92;
        let test_sparsity = 0.07;
        test_state.int_stack.push(test_size);
        test_state.float_stack.push(test_sparsity);
        bool_vector_rand(&mut test_state, &icache());
        if let Some(rbv) = test_state.bool_vector_stack.pop() {
            assert_eq!(rbv.values.len(), test_size as usize);
            assert_eq!(
                rbv.values.iter().filter(|&n| *n == true).count(),
                (test_sparsity * test_size as f32) as usize
            );
        } else {
            assert!(false, "Expected to find bool vector");
        }
    }

    #[test]
    fn bool_vector_rotate_shifts_elements_left() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::from_int_array(vec![1, 1, 1, 1, 0, 0, 0, 0]));
        test_state.bool_stack.push(true);
        bool_vector_rotate(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.get(0).unwrap(),
            &BoolVector::from_int_array(vec![1, 1, 1, 0, 0, 0, 0, 1])
        );
        test_state.bool_stack.push(false);
        bool_vector_rotate(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.get(0).unwrap(),
            &BoolVector::from_int_array(vec![1, 1, 0, 0, 0, 0, 1, 0])
        );
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
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state.int_stack.push(25);
        bool_vector_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[0]; 2:[0]; 3:[1]; 4:[0]; 5:[1];"
        );
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true]));
        test_state.int_stack.push(-2);
        assert_eq!(
            test_state.bool_vector_stack.to_string(),
            "1:[1]; 2:[0]; 3:[0]; 4:[1]; 5:[0]; 6:[1];"
        );
    }

    #[test]
    fn bool_vector_sort_top_item() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true, false, false, true, false]));
        bool_vector_sort_asc(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.to_string(), "1:[0,0,0,1,1];");
        bool_vector_sort_desc(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.to_string(), "1:[1,1,0,0,0];");
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
    fn bool_vector_count_pushes_aggregation_value() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::new(vec![true, false, false, true, false]));
        bool_vector_count(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:2;");
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
    fn bool_vector_zeros_creates_item() {
        let mut test_state = PushState::new();
        let mut test_size = -11;
        test_state.int_stack.push(test_size);
        bool_vector_ones(&mut test_state, &icache());
        assert_eq!(test_state.bool_vector_stack.size(), 0);
        test_size = 11;
        test_state.int_stack.push(test_size);
        bool_vector_ones(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1; test_size as usize])
        );
    }

    /////////////////////////////////////// INTVECTOR //////////////////////////////////////////

    #[test]
    fn int_vector_prints_values() {
        let iv = IntVector::new(vec![1, 2, -3]);
        assert_eq!(iv.to_string(), "[1,2,-3]");
    }

    #[test]
    fn int_vector_bool_index_pushes_indices_of_active_bits() {
        let mut test_state = PushState::new();
        test_state
            .bool_vector_stack
            .push(BoolVector::from_int_array(vec![1, 0, 0, 1, 0]));
        int_vector_bool_index(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![0, 3])
        );
    }

    #[test]
    fn int_vector_get_pushes_vector_element() {
        let test_vec1 = IntVector::new(vec![1, 1, 1, 0, 1, 1, 1, 2]);
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec1);
        test_state.int_stack.push(3);
        int_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 0);
        // Invalid index is changed to valid range
        test_state.int_stack.push(-15);
        int_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 1);
        test_state.int_stack.push(15);
        int_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 2);
    }

    #[test]
    fn int_vector_set_modifies_vector() {
        let test_vec1 = IntVector::new(vec![1, 1, 1, 1, 1, 1, 1, 1]);
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec1);
        test_state.int_stack.push(12); // Second item: new element
        test_state.int_stack.push(5); // Top item: index
        int_vector_set(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![1, 1, 1, 1, 1, 12, 1, 1])
        );
    }

    #[test]
    fn int_vector_add_with_different_overlaps() {
        let test_vec1 = IntVector::new(vec![1, 1, 1, 1, 0, 0, 0, 0]);
        let test_vec2 = IntVector::new(vec![1, 0, 1, 0, 1, 0, 1, 0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec2.clone());
        test_state.int_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(0);
        int_vector_add(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![2, 1, 2, 1, 1, 0, 1, 0])
        );

        // Positive overlap
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec2.clone());
        test_state.int_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(-4);
        int_vector_add(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![1, 0, 1, 0, 1, 0, 1, 0])
        );

        // No overlap
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec2.clone());
        test_state.int_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(8);
        int_vector_add(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![1, 0, 1, 0, 1, 0, 1, 0])
        );
    }

    #[test]
    fn int_vector_subtract_with_partial_overlap() {
        let test_vec1 = IntVector::new(vec![1, 1, 1, 1, 0, 0, 0, 0]);
        let test_vec2 = IntVector::new(vec![1, 0, 1, 0, 1, 0, 1, 0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec2.clone());
        test_state.int_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(4);
        int_vector_subtract(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![1, 0, 1, 0, 0, -1, 0, -1])
        );
    }

    #[test]
    fn int_vector_multiply_with_partial_overlap() {
        let test_vec1 = IntVector::new(vec![-1, -1, -1, -1, 1, 1, 1, 1]);
        let test_vec2 = IntVector::new(vec![1, 2, 1, 2, 1, 2, 1, 2]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec2.clone());
        test_state.int_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(4);
        int_vector_multiply(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![1, 2, 1, 2, -1, -2, -1, -2])
        );
    }

    #[test]
    fn int_vector_divide_with_partial_overlap() {
        let test_vec1 = IntVector::new(vec![1, 2, 1, 2, 1, 2, 1, 2]);
        let test_vec2 = IntVector::new(vec![2, 2, 2, 2, 1, 1, 1, 1]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.int_vector_stack.push(test_vec2.clone());
        test_state.int_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(4);
        int_vector_divide(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 1);
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![2, 2, 2, 2, 1, 0, 1, 0])
        );
    }

    #[test]
    fn int_vector_contains_pushes_to_bool() {
        let mut test_state = PushState::new();
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![3, 4, 1, 2]));
        test_state.int_stack.push(4);
        int_vector_contains(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
        assert_eq!(test_state.int_vector_stack.size(), 1);
        test_state.int_stack.push(5);
        int_vector_contains(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), false);
        assert_eq!(test_state.int_vector_stack.size(), 1);
        assert_eq!(test_state.int_stack.size(), 0);
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
    fn int_vector_from_int_pushes_item() {
        let mut test_state = PushState::new();
        for i in 0..10 {
            test_state.int_stack.push(i);
        }
        test_state.int_stack.push(11);
        int_vector_from_int(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        );
    }

    #[test]
    fn int_vector_ones_creates_item() {
        let mut test_state = PushState::new();
        let mut test_size = -11;
        test_state.int_stack.push(test_size);
        int_vector_ones(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 0);
        test_size = 11;
        test_state.int_stack.push(test_size);
        int_vector_ones(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![1; test_size as usize])
        );
    }

    #[test]
    fn int_vector_rotate_shifts_elements_left() {
        let mut test_state = PushState::new();
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![1, 2, 3, 4, 0, 0, 0, 0]));
        test_state.int_stack.push(5);
        int_vector_rotate(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.get(0).unwrap(),
            &IntVector::new(vec![2, 3, 4, 0, 0, 0, 0, 5])
        );
    }

    #[test]
    fn int_vector_rand_pushes_new_item() {
        let mut test_state = PushState::new();
        let test_size = 92;
        let test_min = -7;
        let test_max = 77;
        test_state.int_stack.push(test_min);
        test_state.int_stack.push(test_max);
        test_state.int_stack.push(test_size);
        int_vector_rand(&mut test_state, &icache());
        if let Some(riv) = test_state.int_vector_stack.pop() {
            assert_eq!(riv.values.len(), test_size as usize);
            assert_eq!(
                riv.values
                    .iter()
                    .filter(|&n| (*n >= test_min && *n <= test_max) == true)
                    .count(),
                test_size as usize
            );
        } else {
            assert!(false, "Expected to find bool vector");
        }
    }

    #[test]
    fn int_vector_set_insert_does_not_allow_duplicates() {
        let mut test_state = PushState::new();
        let test_input = IntVector::new(vec![1,2,3,4]);
        test_state.int_vector_stack.push(test_input.clone());
        test_state.int_stack.push(1);
        int_vector_set_insert(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.get(0).unwrap(), &test_input);
        test_state.int_stack.push(5);
        int_vector_set_insert(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.get(0).unwrap(), &IntVector::new(vec![1,2,3,4,5]));
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
    fn int_vector_sort_top_item() {
        let mut test_state = PushState::new();
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![34, 0, -28, 111, -1]));
        int_vector_sort_asc(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[-28,-1,0,34,111];"
        );
        int_vector_sort_desc(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            "1:[111,34,0,-1,-28];"
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
    fn int_vector_sum_pushes_aggregation_value() {
        let mut test_state = PushState::new();
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![1, 3, -2, 5, 7]));
        int_vector_sum(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:14;");
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
    fn int_vector_zeros_creates_item() {
        let mut test_state = PushState::new();
        let mut test_size = -11;
        test_state.int_stack.push(test_size);
        int_vector_zeros(&mut test_state, &icache());
        assert_eq!(test_state.int_vector_stack.size(), 0);
        test_size = 11;
        test_state.int_stack.push(test_size);
        int_vector_zeros(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.pop().unwrap(),
            IntVector::new(vec![0; test_size as usize])
        );
    }

    ////////////////////////////////////// FLOATVECTOR //////////////////////////////////////////

    #[test]
    fn float_vector_prints_values() {
        let fv = FloatVector::new(vec![1.2, 3.4, -4.5]);
        assert_eq!(fv.to_string(), "[1.2,3.4,-4.5]");
    }

    #[test]
    fn float_vector_get_pushes_vector_element() {
        let test_vec1 = FloatVector::new(vec![2.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 4.0]);
        let mut test_state = PushState::new();
        test_state.float_vector_stack.push(test_vec1);
        test_state.int_stack.push(3);
        float_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 0.0);
        // Invalid index is changed to valid index
        test_state.int_stack.push(-115);
        float_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 2.0);
        test_state.int_stack.push(15);
        float_vector_get(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 4.0);
    }

    #[test]
    fn float_vector_set_modifies_vector() {
        let test_vec1 = FloatVector::new(vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]);
        let mut test_state = PushState::new();
        test_state.float_vector_stack.push(test_vec1);
        test_state.float_stack.push(12.0);
        test_state.int_stack.push(5); // Top item: index
        float_vector_set(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![1.0, 1.0, 1.0, 1.0, 1.0, 12.0, 1.0, 1.0])
        );
    }

    #[test]
    fn float_vector_add_with_partial() {
        let test_vec1 = FloatVector::new(vec![1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
        let test_vec2 = FloatVector::new(vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.float_vector_stack.push(test_vec2.clone());
        test_state.float_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(0);
        float_vector_add(&mut test_state, &icache());
        assert_eq!(test_state.float_vector_stack.size(), 1);
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![2.0, 1.0, 2.0, 1.0, 1.0, 0.0, 1.0, 0.0])
        );
    }

    #[test]
    fn float_vector_sine_generates_2pi_angle() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(1000); // Array length
        test_state.float_stack.push(0.0); // Phase angle is 0
        test_state.float_stack.push(0.001); // Angle velocity
        test_state.float_stack.push(1.0); // Amplitude
        float_vector_sine(&mut test_state, &icache());

        let sine_vector = test_state.float_vector_stack.pop().unwrap().values;
        assert_eq!(sine_vector.len(), 1000);
        assert!(f32::abs(sine_vector[0]) < 0.01f32);
        assert!(f32::abs(sine_vector[249] - 1.0) < 0.01f32);
        assert!(f32::abs(sine_vector[499]) < 0.01f32);
        assert!(f32::abs(sine_vector[749] + 1.0) < 0.01f32);
        assert!(f32::abs(sine_vector[999]) < 0.01f32);
    }

    #[test]
    fn float_vector_subtract_with_partial_overlap() {
        let test_vec1 = FloatVector::new(vec![1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
        let test_vec2 = FloatVector::new(vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.float_vector_stack.push(test_vec2.clone());
        test_state.float_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(4);
        float_vector_subtract(&mut test_state, &icache());
        assert_eq!(test_state.float_vector_stack.size(), 1);
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![1.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, -1.0])
        );
    }

    #[test]
    fn float_vector_multiply_with_partial_overlap() {
        let test_vec1 = FloatVector::new(vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0]);
        let test_vec2 = FloatVector::new(vec![1.0, 3.0, 1.0, 3.0, 1.0, 3.0, 1.0, 3.0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.float_vector_stack.push(test_vec2.clone());
        test_state.float_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(4);
        float_vector_multiply(&mut test_state, &icache());
        assert_eq!(test_state.float_vector_stack.size(), 1);
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![1.0, 3.0, 1.0, 3.0, 2.0, 6.0, 2.0, 6.0])
        );
    }

    #[test]
    fn float_vector_multiply_scalar_to_each_element() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        float_vector_ones(&mut test_state, &icache());
        test_state.float_stack.push(3.0);
        float_vector_multiply_scalar(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![3.0, 3.0, 3.0, 3.0])
        );
    }

    #[test]
    fn float_vector_divide_with_partial_overlap() {
        let test_vec1 = FloatVector::new(vec![2.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0]);
        let test_vec2 = FloatVector::new(vec![6.0, 4.0, 6.0, 4.0, 6.0, 4.0, 6.0, 4.0]);

        // Full overlap
        let mut test_state = PushState::new();
        test_state.float_vector_stack.push(test_vec2.clone());
        test_state.float_vector_stack.push(test_vec1.clone());
        test_state.int_stack.push(4);
        float_vector_divide(&mut test_state, &icache());
        assert_eq!(test_state.float_vector_stack.size(), 1);
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![6.0, 4.0, 6.0, 4.0, 3.0, 2.0, 3.0, 2.0])
        );
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
    fn float_vector_ones_creates_item() {
        let mut test_state = PushState::new();
        let mut test_size = -11;
        test_state.int_stack.push(test_size);
        float_vector_ones(&mut test_state, &icache());
        assert_eq!(test_state.float_vector_stack.size(), 0);
        test_size = 11;
        test_state.int_stack.push(test_size);
        float_vector_ones(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![1.0; test_size as usize])
        );
    }

    #[test]
    fn float_vector_rand_pushes_new_item() {
        let mut test_state = PushState::new();
        let test_size = 1000;
        let test_mean = vec![-7.0, 0.0, 12.0];
        let test_stddev = vec![0.77, 1.23];
        for tm in &test_mean {
            for ts in &test_stddev {
                test_state.int_stack.push(test_size);
                test_state.float_stack.push(*ts);
                test_state.float_stack.push(*tm);
                float_vector_rand(&mut test_state, &icache());
                if let Some(fvs) = test_state.float_vector_stack.pop() {
                    assert_eq!(fvs.values.len(), test_size as usize);
                    let sum = fvs.values.iter().sum::<f32>();
                    let count = fvs.values.len() as f32;
                    assert!(f32::abs(sum / count - tm) < *ts);
                } else {
                    assert!(false, "Expected to find bool vector");
                }
            }
        }
    }

    #[test]
    fn float_vector_sort_top_item() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![34.2, 0.0, -28.1, 111.1, -1.5]));
        float_vector_sort_asc(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[-28.1,-1.5,0,34.2,111.1];"
        );
        float_vector_sort_desc(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.to_string(),
            "1:[111.1,34.2,0,-1.5,-28.1];"
        );
    }

    #[test]
    fn float_vector_rotate_shifts_elements_left() {
        let mut test_state = PushState::new();
        test_state.float_vector_stack.push(FloatVector::new(vec![
            1.0, 2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0,
        ]));
        test_state.float_stack.push(5.0);
        float_vector_rotate(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.get(0).unwrap(),
            &FloatVector::new(vec![2.0, 3.0, 4.0, 0.0, 0.0, 0.0, 0.0, 5.0])
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
    fn float_vector_sum_pushes_aggregation_value() {
        let mut test_state = PushState::new();
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![1.0, 3.0, -2.0, 5.0, 7.0]));
        float_vector_sum(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "1:14;");
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

    #[test]
    fn float_vector_zeros_creates_item() {
        let mut test_state = PushState::new();
        let mut test_size = -11;
        test_state.int_stack.push(test_size);
        float_vector_zeros(&mut test_state, &icache());
        assert_eq!(test_state.float_vector_stack.size(), 0);
        test_size = 11;
        test_state.int_stack.push(test_size);
        float_vector_zeros(&mut test_state, &icache());
        assert_eq!(
            test_state.float_vector_stack.pop().unwrap(),
            FloatVector::new(vec![0.0; test_size as usize])
        );
    }
}
