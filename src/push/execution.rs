use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::state::PushState;
use crate::push::state::*;
use std::collections::HashMap;
use std::process::Command;

/// Code queued for execution. The EXEC stack maintains the execution state of the Push
/// interpreter. Instructions that specifically manipulate the EXEC stack can be used to implement
/// various kinds of control structures. The CODE stack can also be used in this way, but
/// manipulations to the EXEC stack are "live" in the sense that they are manipulating the actual
/// execution state of the interpreter, not just code that might later be executed.
pub fn load_exec_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("EXEC.="), Instruction::new(exec_eq));
    map.insert(String::from("EXEC.CMD"), Instruction::new(exec_cmd));
    map.insert(String::from("EXEC.DEFINE"), Instruction::new(exec_define));
    map.insert(String::from("EXEC.LOOP"), Instruction::new(exec_loop));
    map.insert(String::from("EXEC.DUP"), Instruction::new(exec_dup));
    map.insert(String::from("EXEC.FLUSH"), Instruction::new(exec_flush));
    map.insert(String::from("EXEC.ID"), Instruction::new(exec_id));
    map.insert(String::from("EXEC.IF"), Instruction::new(exec_if));
    map.insert(String::from("EXEC.K"), Instruction::new(exec_k));
    map.insert(String::from("EXEC.POP"), Instruction::new(exec_pop));
    map.insert(String::from("EXEC.ROT"), Instruction::new(exec_rot));
    map.insert(String::from("EXEC.S"), Instruction::new(exec_s));
    map.insert(String::from("EXEC.SHOVE"), Instruction::new(exec_shove));
    map.insert(
        String::from("EXEC.STACKDEPTH"),
        Instruction::new(exec_stack_depth),
    );
    map.insert(String::from("EXEC.SWAP"), Instruction::new(exec_swap));
    map.insert(String::from("EXEC.Y"), Instruction::new(exec_y));
    map.insert(String::from("EXEC.YANK"), Instruction::new(exec_yank));
    map.insert(
        String::from("EXEC.YANKDUP"),
        Instruction::new(exec_yank_dup),
    );
}

/// EXEC.ID: Pushes the ID of the EXEC stack to the INTEGER stack.
pub fn exec_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(EXEC_STACK_ID);
}

/// EXEC.CMD: Executes the top item of the name stack on the command line.
pub fn exec_cmd(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(cmd) = push_state.name_stack.pop() {
        Command::new(cmd).spawn();
    }
}

/// EXEC.=: Pushes TRUE if the top two items on the EXEC stack are equal, or FALSE otherwise.
pub fn exec_eq(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.exec_stack.copy_vec(2) {
        push_state
            .bool_stack
            .push(pv[0].to_string() == pv[1].to_string());
    }
}

/// EXEC.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the top
/// item of the EXEC stack back onto the EXEC stack.
pub fn exec_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(instruction) = push_state.exec_stack.pop() {
            push_state.name_bindings.insert(name, instruction);
        }
    }
}

/// EXEC.LOOP: An iteration instruction that executes the top item on the EXEC stack a number
/// of times that depends on the top two INDEX items, while also pushing the loop counter onto the
/// INDEX stack for possible access during the execution of the body of the loop.
/// First the code and the index arguments are saved locally and popped. Then the current and the
/// destination field of the index are compared. If they are equal nothing happens, i.e. the
/// index pair is just removed and the loop is terminated.
/// If the integers are not equal then the current index will be
/// pushed onto the INDEX stack but two items will be pushed onto the EXEC stack -- first a
/// recursive call to EXEC.LOOP (with the same code and destination index, but with a current
/// index that has been incremented by 1 to be closer to the destination
/// index) and then the body code.
pub fn exec_loop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(body) = push_state.exec_stack.pop() {
        if let Some(index) = push_state.index_stack.copy(0) {
            if index.current < index.destination {
                let updated_loop = Item::list(vec![
                    body.clone(),
                    Item::instruction("EXEC.LOOP".to_string()),
                    Item::instruction("INDEX.INCREASE".to_string()),
                ]);
                push_state.exec_stack.push(updated_loop);
                push_state.exec_stack.push(body);
            } else {
                push_state.index_stack.pop();
            }
        }
    }
}

/// EXEC.DUP: Duplicates the top item on the EXEC stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!). This may be thought of as a "DO TWICE"
/// instruction.
pub fn exec_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(instruction) = push_state.exec_stack.copy(0) {
        push_state.exec_stack.push(instruction);
    }
}

/// EXEC.FLUSH: Empties the EXEC stack. This may be thought of as a "HALT" instruction.
pub fn exec_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.exec_stack.flush();
}

/// EXEC.IF: If the top item of the BOOLEAN stack is TRUE then this removes the second item on the
/// EXEC stack, leaving the first item to be executed. If it is false then it removes the first
/// item, leaving the second to be executed. This is similar to CODE.IF except that it operates on
/// the EXEC stack. This acts as a NOOP unless there are at least two items on the EXEC stack and
/// one item on the BOOLEAN stack.
pub fn exec_if(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.exec_stack.pop_vec(2) {
        if let Some(exec_first) = push_state.bool_stack.pop() {
            if exec_first {
                // Push first element for execution
                push_state.exec_stack.push(code[1].clone());
            } else {
                // Push second element for execution
                push_state.exec_stack.push(code[0].clone());
            }
        }
    }
}

/// EXEC.K: The Push implementation of the "K combinator". Removes the second item on the EXEC
/// stack.
pub fn exec_k(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.exec_stack.pop_vec(2) {
        push_state.exec_stack.push(code[1].clone());
    }
}

/// EXEC.POP: Pops the EXEC stack. This may be thought of as a "DONT" instruction.
pub fn exec_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.exec_stack.pop();
}

/// EXEC.ROT: Rotates the top three items on the EXEC stack, pulling the third item out and pushing
/// it on top. This is equivalent to "2 EXEC.YANK".
pub fn exec_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.exec_stack.yank(2);
}

/// EXEC.S: The Push implementation of the "S combinator". Pops 3 items from the EXEC stack, which
/// we will call A, B, and C (with A being the first one popped). Then pushes a list containing B
/// and C back onto the EXEC stack, followed by another instance of C, followed by another instance
/// of A.
pub fn exec_s(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(code) = push_state.exec_stack.pop_vec(3) {
        let a = &code[2];
        let b = &code[1];
        let c = &code[0];
        let bc = Item::list(vec![c.clone(), b.clone()]);
        push_state.exec_stack.push(bc);
        push_state.exec_stack.push(c.clone());
        push_state.exec_stack.push(a.clone());
    }
}

/// EXEC.SHOVE: Inserts the top EXEC item "deep" in the stack, at the position indexed by the top
/// INTEGER. This may be thought of as a "DO LATER" instruction.
pub fn exec_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.exec_stack.size() as i32) - 1, shove_index),
            0,
        ) as usize;
        push_state.exec_stack.shove(corr_index as usize);
    }
}

/// EXEC.STACKDEPTH: Pushes the stack depth onto the INTEGER stack.
pub fn exec_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.exec_stack.size() as i32);
}

/// EXEC.SWAP: Swaps the top two items on the EXEC stack.
pub fn exec_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.exec_stack.shove(1);
}

/// EXEC.Y: The Push implementation of the "Y combinator". Inserts beneath the top item of the EXEC
/// stack a new item of the form "( EXEC.Y <TopItem> )".
pub fn exec_y(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(top_item) = push_state.exec_stack.copy(0) {
        push_state.exec_stack.push(Item::list(vec![
            top_item,
            Item::instruction("EXEC.Y".to_string()),
        ]));
        push_state.exec_stack.shove(1);
    }
}

/// EXEC.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the stack.
/// The index is taken from the INTEGER stack. This may be thought of as a "DO SOONER" instruction.
pub fn exec_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.exec_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        push_state.exec_stack.yank(corr_index);
    }
}

/// EXEC.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the stack,
/// without removing the deep item. The index is taken from the INTEGER stack.
pub fn exec_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.exec_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        if let Some(deep_item) = push_state.exec_stack.copy(corr_index as usize) {
            push_state.exec_stack.push(deep_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::index::Index;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn exec_eq_pushes_true_when_elements_equal() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(1));
        test_state.exec_stack.push(Item::int(1));
        exec_eq(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.size(), 2);
        assert_eq!(test_state.bool_stack.to_string(), "TRUE");
    }

    #[test]
    fn exec_eq_pushes_false_when_elements_unequal() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(1));
        test_state.exec_stack.push(Item::int(2));
        exec_eq(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.size(), 2);
        assert_eq!(test_state.bool_stack.to_string(), "FALSE");
    }

    #[test]
    fn exec_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(2));
        test_state.name_stack.push(String::from("TEST"));
        exec_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::int(2).to_string()
        );
    }

    #[test]
    fn exec_loop_pushes_body_and_updated_loop() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::noop());
        test_state.index_stack.push(Index::new(3));
        exec_loop(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "NOOP ( INDEX.INCREASE EXEC.LOOP NOOP )");
    }

    #[test]
    fn exec_loop_removes_index_when_terminated() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::noop());
        let mut test_index = Index::new(3);
        test_index.current = 3;
        test_state.index_stack.push(test_index);
        exec_loop(&mut test_state, &icache());
        assert_eq!(test_state.index_stack.to_string(), "");
        assert_eq!(test_state.exec_stack.to_string(), "");
    }

    #[test]
    fn exec_dup_duplicates_top_element() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::noop());
        exec_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "NOOP NOOP"
        );
    }

    #[test]
    fn exec_flush_empties_stack() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .exec_stack
            .push(Item::list(vec![Item::int(0), Item::int(2)]));
        test_state
            .exec_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        exec_flush(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
    }

    #[test]
    fn exec_if_pushes_first_item_when_true() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        exec_if(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "1");
        assert_eq!(test_state.bool_stack.to_string(), "");
    }

    #[test]
    fn exec_if_pushes_second_item_when_false() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(false);
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        exec_if(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "2");
        assert_eq!(test_state.bool_stack.to_string(), "");
    }

    #[test]
    fn exec_k_removes_second_item() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(false);
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        exec_k(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "1");
    }

    #[test]
    fn exec_pop_removes_first_item() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(false);
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        exec_pop(&mut test_state, &icache());
        assert_eq!(test_state.exec_stack.to_string(), "2");
    }

    #[test]
    fn exec_rot_shuffles_elements() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(3));
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1 2 3"
        );
        exec_rot(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "3 1 2"
        );
    }

    #[test]
    fn exec_s_pushes_elements_in_right_order() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(3));
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1 2 3"
        );
        exec_s(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1 3 ( 2 3 )"
        );
    }

    #[test]
    fn exec_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(4));
        test_state.exec_stack.push(Item::int(3));
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1 2 3 4"
        );
        test_state.int_stack.push(2);
        exec_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "2 3 1 4"
        );
    }

    #[test]
    fn exec_stack_depth_pushes_size() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .exec_stack
            .push(Item::list(vec![Item::int(0), Item::int(2)]));
        test_state
            .exec_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        exec_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "2");
    }

    #[test]
    fn exec_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(0));
        test_state.exec_stack.push(Item::int(1));
        exec_swap(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "0 1"
        );
    }

    #[test]
    fn exec_y_inserts_y_copy_beneath_top_element() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(0));
        exec_y(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "0 ( EXEC.Y 0 )"
        );
    }

    #[test]
    fn exec_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(5));
        test_state.exec_stack.push(Item::int(4));
        test_state.exec_stack.push(Item::int(3));
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1 2 3 4 5"
        );
        test_state.int_stack.push(3);
        exec_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "4 1 2 3 5"
        );
    }

    #[test]
    fn exec_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.exec_stack.push(Item::int(5));
        test_state.exec_stack.push(Item::int(4));
        test_state.exec_stack.push(Item::int(3));
        test_state.exec_stack.push(Item::int(2));
        test_state.exec_stack.push(Item::int(1));
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1 2 3 4 5"
        );
        test_state.int_stack.push(3);
        exec_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "4 1 2 3 4 5"
        );
    }
}
