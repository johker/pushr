use crate::push::atoms::Atom;
use crate::push::instructions::Instruction;
use crate::push::stack::PushStack;
use crate::push::state::PushState;
use std::collections::HashMap;

/// Maps the default code instructions to their identifiers
pub fn load_code_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("CODE.="), Instruction::new(code_eq));
    map.insert(String::from("CODE.APPEND"), Instruction::new(code_append));
    map.insert(String::from("CODE.ATOM"), Instruction::new(code_atom));
    map.insert(String::from("CODE.CAR"), Instruction::new(code_first));
    map.insert(String::from("CODE.CDR"), Instruction::new(code_rest));
    map.insert(String::from("CODE.CONS"), Instruction::new(code_cons));
    map.insert(
        String::from("CODE.CONTAINER"),
        Instruction::new(code_container),
    );
    map.insert(
        String::from("CODE.CONTAINS"),
        Instruction::new(code_contains),
    );
    map.insert(String::from("CODE.DEFINE"), Instruction::new(code_define));
    map.insert(
        String::from("CODE.DEFINITION"),
        Instruction::new(code_definition),
    );
    map.insert(
        String::from("CODE.DISCREPANCY"),
        Instruction::new(code_discrepancy),
    );
    map.insert(String::from("CODE.DO"), Instruction::new(code_do));
    map.insert(String::from("CODE.DO*"), Instruction::new(code_pop_and_do));
    map.insert(
        String::from("CODE.DO*COUNT"),
        Instruction::new(code_do_count),
    );
    map.insert(
        String::from("CODE.DO*RANGE"),
        Instruction::new(code_do_range),
    );
    map.insert(
        String::from("CODE.DO*TIMES"),
        Instruction::new(code_do_times),
    );
    map.insert(String::from("CODE.DUP"), Instruction::new(code_dup));
    map.insert(String::from("CODE.EXTRACT"), Instruction::new(code_extract));
    map.insert(String::from("CODE.FLUSH"), Instruction::new(code_flush));
    map.insert(
        String::from("CODE.FROMBOOLEAN"),
        Instruction::new(code_from_bool),
    );
    map.insert(
        String::from("CODE.FROMFLOAT"),
        Instruction::new(code_from_float),
    );
    map.insert(
        String::from("CODE.FROMINTEGER"),
        Instruction::new(code_from_int),
    );
    map.insert(
        String::from("CODE.FROMNAME"),
        Instruction::new(code_from_name),
    );
}

//
// ------------------ Type: BOOLEAN ---------------------
//

/// CODE.=: Pushes TRUE if the top two pieces of CODE are equal,
/// or FALSE otherwise.
pub fn code_eq(push_state: &mut PushState) {
    if let Some(pv) = push_state.code_stack.observe_vec(2) {
        push_state
            .bool_stack
            .push(pv[0].to_string() == pv[1].to_string());
    }
}

/// CODE.APPEND: Pushes the result of appending the top two pieces of code.
/// If one of the pieces of code is a single instruction or literal (that is,
/// something not surrounded by parentheses) then it is surrounded by
/// parentheses first.
pub fn code_append(push_state: &mut PushState) {
    if let Some(pv) = push_state.code_stack.pop_vec(2) {
        push_state.code_stack.push(Atom::List {
            atoms: PushStack::from_vec(pv),
        });
    }
}

/// CODE.ATOM: Pushes TRUE onto the BOOLEAN stack if the top piece of code is a single instruction
/// or a literal, and FALSE otherwise (that is, if it is something surrounded by parentheses).
pub fn code_atom(push_state: &mut PushState) {
    // Equality only checks type and ignores value
    push_state.bool_stack.push(
        push_state.code_stack.last_eq(&Atom::int(0))
            || push_state.code_stack.last_eq(&Atom::noop()),
    );
}

/// CODE.CAR: Pushes the first item of the list on top of the CODE stack. For example, if the top
/// piece of code is "( A B )" then this pushes "A" (after popping the argument). If the code on
/// top of the stack is not a list then this has no effect. The name derives from the similar Lisp
/// function; a more generic name would be "FIRST".
pub fn code_first(push_state: &mut PushState) {
    if push_state.code_stack.last_eq(&Atom::empty_list()) {
        match push_state.code_stack.pop() {
            Some(Atom::List { mut atoms }) => {
                if let Some(atom) = atoms.pop() {
                    push_state.code_stack.push(atom);
                }
            }
            _ => (),
        }
    }
}

/// CODE.CDR: Pushes a version of the list from the top of the CODE stack without its first
/// element. For example, if the top piece of code is "( A B )" then this pushes "( B )" (after
/// popping the argument). If the code on top of the stack is not a list then this pushes the empty
/// list ("( )"). The name derives from the similar Lisp function; a more generic name would be
/// "REST".
pub fn code_rest(push_state: &mut PushState) {
    println!("{}", push_state.code_stack.to_string());
    match push_state.code_stack.pop() {
        Some(Atom::List { mut atoms }) => {
            atoms.pop();
            push_state.code_stack.push(Atom::List { atoms: atoms });
        }
        _ => (),
    }
}

/// CODE.CONS: Pushes the result of "consing" (in the Lisp sense) the second stack item onto the
/// first stack item (which is coerced to a list if necessary). For example, if the top piece of
/// code is "( A B )" and the second piece of code is "X" then this pushes "( X A B )" (after
/// popping the argument).
pub fn code_cons(push_state: &mut PushState) {
    if let Some(pv) = push_state.code_stack.pop_vec(2) {
        let mut consblock = PushStack::new();
        for i in (0..2).rev() {
            match &pv[i] {
                Atom::Literal { push_type: _ } => {
                    consblock.push(pv[i].clone());
                }
                Atom::List { atoms: a } => {
                    if let Some(vec) = a.observe_vec(a.size()) {
                        consblock.push_vec(vec)
                    }
                }
                _ => (),
            }
        }
        push_state.code_stack.push(Atom::List { atoms: consblock });
    }
}

/// CODE.CONTAINER: Pushes the "container" of the second CODE stack item within the first CODE
/// stack item onto the CODE stack. If second item contains the first anywhere (i.e. in any nested
/// list) then the container is the smallest sub-list that contains but is not equal to the first
/// instance. For example, if the top piece of code is "( B ( C ( A ) ) ( D ( A ) ) )" and the
/// second piece of code is "( A )" then this pushes ( C ( A ) ). Pushes an empty list if there is
/// no such container.
pub fn code_container(push_state: &mut PushState) {
    if let Some(ov) = push_state.code_stack.observe_vec(2) {
        let stack_str = push_state.code_stack.to_string();
        let first_el = ov[1].to_string();
        let code_str = ov[0].to_string();
        if first_el.contains(&code_str) {
            println!("Top element {} contains second element {}", ov[1], ov[0]);
            let split = stack_str.split(&code_str);
            let vec: Vec<&str> = split.collect();
            let split = vec[0].split(" ");
            let vec: Vec<&str> = split.collect();
            let mut index_vector = Vec::new();
            for x in vec.iter() {
                let k = x.chars().nth(0).unwrap();
                index_vector.push(k.to_digit(10).unwrap());
                if x.ends_with(";;") {
                    // Remove sub list indices
                    while index_vector.pop().unwrap() != 1 {}
                }
            }
            let mut container = ov[1].clone();
            for (i, x) in index_vector.iter().enumerate() {
                // If the next index is 1 a new sublist begins
                // This sublist is our new container
                if i == 0 || i == index_vector.len() - 1 || index_vector[i + 1] != 1 {
                    continue;
                }
                match container {
                    Atom::List { atoms: sublist } => {
                        container = sublist.observe((*x as usize) - 1).unwrap();
                    }
                    _ => println!("Unexpected element - container: {}", container.to_string()),
                }
            }
            // Push smallest container of fist atom to the stack
            push_state.code_stack.push(container);
        } else {
            // Push empty list if second atom is not part of first
            push_state.code_stack.push(Atom::empty_list());
        }
    }
}

/// CODE.CONTAINS: Pushes TRUE on the BOOLEAN stack if the second CODE stack item contains the
/// first CODE stack item anywhere (e.g. in a sub-list).
pub fn code_contains(push_state: &mut PushState) {
    if let Some(ov) = push_state.code_stack.observe_vec(2) {
        let first_el = ov[1].to_string();
        let code_str = ov[0].to_string();
        if first_el.contains(&code_str) {
            push_state.bool_stack.push(true);
        } else {
            push_state.bool_stack.push(false);
        }
    }
}

/// CODE.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the top
/// item of the CODE stack onto the EXEC stack.
pub fn code_define(push_state: &mut PushState) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(instruction) = push_state.code_stack.pop() {
            push_state.name_bindings.insert(name, instruction);
        }
    }
}

/// CODE.DEFINITION: Pushes the definition associated with the top NAME on the NAME stack (if any)
/// onto the CODE stack. This extracts the definition for inspection/manipulation, rather than for
/// immediate execution (although it may then be executed with a call to CODE.DO or a similar
/// instruction).
pub fn code_definition(push_state: &mut PushState) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(instruction) = push_state.name_bindings.get(name) {
            push_state.code_stack.push(instruction.clone());
        }
    }
}
/// CODE.DISCREPANCY: Pushes a measure of the discrepancy between the top two CODE stack items onto
/// the INTEGER stack. This will be zero if the top two items are equivalent, and will be higher
/// the 'more different' the items are from one another. The calculation is as follows:
/// 1. Construct a list of all of the unique items in both of the lists (where uniqueness is
///    determined by equalp). Sub-lists and atoms all count as items.
/// 2. Initialize the result to zero.
/// 3. For each unique item increment the result by the difference between the number of
///    occurrences of the item in the two pieces of code.
/// 4. Push the result.
pub fn code_discrepancy(push_state: &mut PushState) {
    let mut discrepancy = 0;
    if let Some(ov) = push_state.code_stack.observe_vec(2) {
        match &ov[0] {
            Atom::List { atoms: fstlist } => {
                match &ov[1] {
                    Atom::List { atoms: scdlist } => {
                        // Compare lists
                        if let Some(fstvec) = fstlist.observe_vec(fstlist.size()) {
                            for (i, x) in fstvec.iter().rev().enumerate() {
                                if let Some(val) = scdlist.equal_at(i, x) {
                                    if !val {
                                        discrepancy += 1;
                                    }
                                }
                            }
                        }
                        discrepancy =
                            discrepancy + (fstlist.size() as i32 - scdlist.size() as i32).abs();
                    }
                    _ => {
                        discrepancy = if ov[0].to_string() != ov[1].to_string() {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
            _ => {
                discrepancy = if ov[0].to_string() != ov[1].to_string() {
                    1
                } else {
                    0
                }
            }
        }
        push_state.int_stack.push(discrepancy);
    }
}

/// CODE.DO: Recursively invokes the interpreter on the program on top of the CODE stack. After
/// evaluation the CODE stack is popped; normally this pops the program that was just executed, but
/// if the expression itself manipulates the stack then this final pop may end up popping something
/// else.
pub fn code_do(push_state: &mut PushState) {
    if let Some(instruction) = push_state.code_stack.observe(0) {
        push_state
            .exec_stack
            .push(Atom::InstructionMeta { name: "CODE.POP" });
        push_state.exec_stack.push(instruction);
    }
}

/// CODE.DO*: Like CODE.DO but pops the stack before, rather than after, the recursive execution.
pub fn code_pop_and_do(push_state: &mut PushState) {
    if let Some(instruction) = push_state.code_stack.observe(0) {
        push_state.exec_stack.push(instruction);
        push_state
            .exec_stack
            .push(Atom::InstructionMeta { name: "CODE.POP" });
    }
}

/// CODE.DO*COUNT: An iteration instruction that performs a loop (the body of which is taken from
/// the CODE stack) the number of times indicated by the INTEGER argument, pushing an index (which
/// runs from zero to one less than the number of iterations) onto the INTEGER stack prior to each
/// execution of the loop body. This should be implemented as a macro that expands into a call to
/// CODE.DO*RANGE.
/// CODE.DO*COUNT takes a single INTEGER argument (the number of times that the loop will be
/// executed) and a single CODE argument (the body of the loop). If the provided INTEGER argument
/// is negative or zero then this becomes a NOOP. Otherwise it expands into:
/// ( 0 <1 - IntegerArg> CODE.QUOTE <CodeArg> CODE.DO*RANGE )
pub fn code_do_count(_push_state: &mut PushState) {
    // TODO
}

/// CODE.DO*RANGE: An iteration instruction that executes the top item on the CODE stack a number
/// of times that depends on the top two integers, while also pushing the loop counter onto the
/// INTEGER stack for possible access during the execution of the body of the loop. The top integer
/// is the "destination index" and the second integer is the "current index." First the code and
/// the integer arguments are saved locally and popped. Then the integers are compared. If the
/// integers are equal then the current index is pushed onto the INTEGER stack and the code (which
/// is the "body" of the loop) is pushed onto the EXEC stack for subsequent execution. If the
/// integers are not equal then the current index will still be pushed onto the INTEGER stack but
/// two items will be pushed onto the EXEC stack -- first a recursive call to CODE.DO*RANGE (with
/// the same code and destination index, but with a current index that has been either incremented
/// or decremented by 1 to be closer to the destination index) and then the body code. Note that
/// the range is inclusive of both endpoints; a call with integer arguments 3 and 5 will cause its
/// body to be executed 3 times, with the loop counter having the values 3, 4, and 5. Note also
/// that one can specify a loop that "counts down" by providing a destination index that is less
/// than the specified current index.
pub fn code_do_range(push_state: &mut PushState) {
    if let Some(body) = push_state.code_stack.pop() {
        if let Some(indices) = push_state.int_stack.pop_vec(2) {
            let destination_idx = indices[1];
            let mut current_idx = indices[0];
            if current_idx == destination_idx {
                push_state.int_stack.push(current_idx);
                push_state.exec_stack.push(body);
            } else {
                push_state.exec_stack.push(Atom::InstructionMeta {
                    name: "CODE.DO*RANGE",
                });
                push_state.exec_stack.push(body);
                if current_idx < destination_idx {
                    current_idx += 1;
                } else {
                    current_idx -= 1;
                }
                push_state.int_stack.push(current_idx);
                push_state.int_stack.push(destination_idx);
            }
        }
    }
}

/// CODE.DO*TIMES: Like CODE.DO*COUNT but does not push the loop counter. This should be
/// implemented as a macro that expands into CODE.DO*RANGE, similarly to the implementation of
/// CODE.DO*COUNT, except that a call to INTEGER.POP should be tacked on to the front of the loop
/// body code in the call to CODE.DO*RANGE. This call to INTEGER.POP will remove the loop counter,
/// which will have been pushed by CODE.DO*RANGE, prior to the execution of the loop body.
pub fn code_do_times(_push_state: &mut PushState) {
    // TODO
}

/// CODE.DUP: Duplicates the top item on the CODE stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!).
pub fn code_dup(push_state: &mut PushState) {
    if let Some(instruction) = push_state.code_stack.observe(0) {
        push_state.code_stack.push(instruction);
    }
}

/// CODE.DUP: Duplicates the top item on the CODE stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!).
pub fn code_extract(_push_state: &mut PushState) {
    // TODO
}

/// CODE.FLUSH: Empties the CODE stack.
pub fn code_flush(push_state: &mut PushState) {
    push_state.code_stack.flush();
}

/// CODE.FROMBOOLEAN: Pops the BOOLEAN stack and pushes the popped item (TRUE or FALSE) onto the
/// CODE stack.
pub fn code_from_bool(push_state: &mut PushState) {
    if let Some(bval) = push_state.bool_stack.pop() {
        push_state.code_stack.push(Atom::bool(bval));
    }
}
/// CODE.FROMFLOAT: Pops the FLOAT stack and pushes the popped item onto the CODE stack.
pub fn code_from_float(push_state: &mut PushState) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.code_stack.push(Atom::float(fval));
    }
}

/// CODE.FROMINTEGER: Pops the INTEGER stack and pushes the popped integer onto the CODE stack.
pub fn code_from_int(push_state: &mut PushState) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.code_stack.push(Atom::int(ival));
    }
}

/// CODE.FROMNAME: Pops the NAME stack and pushes the popped item onto the CODE stack.
pub fn code_from_name(push_state: &mut PushState) {
    if let Some(nval) = push_state.name_stack.pop() {
        push_state.code_stack.push(Atom::id(nval));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_eq_pushes_true_when_elements_equal() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(1));
        test_state.code_stack.push(Atom::int(1));
        code_eq(&mut test_state);
        assert_eq!(test_state.code_stack.size(), 2);
        if let Some(val) = test_state.bool_stack.pop() {
            assert_eq!(val, true, "Must be true in case of equality");
        } else {
            assert!(false, "Expected bool value");
        }
    }

    #[test]
    fn code_eq_pushes_false_when_elements_unequal() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(1));
        test_state.code_stack.push(Atom::int(2));
        code_eq(&mut test_state);
        assert_eq!(test_state.code_stack.size(), 2);
        if let Some(val) = test_state.bool_stack.pop() {
            assert_eq!(val, false, "Must be false in case of inequality");
        } else {
            assert!(false, "Expected bool value");
        }
    }

    #[test]
    fn code_append_pushes_block_when_finding_literals() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(1));
        test_state.code_stack.push(Atom::int(2));
        code_append(&mut test_state);
        assert_eq!(test_state.code_stack.size(), 1, "Excpected single element");
        assert!(
            test_state.code_stack.last_eq(&Atom::empty_list()),
            "Expected Code Block"
        );
    }

    #[test]
    fn code_atom_pushes_true_when_no_list_found() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(0));
        code_atom(&mut test_state);
        assert!(
            test_state.bool_stack.last_eq(&true),
            "Should push true for Literal"
        );
        test_state = PushState::new();
        test_state.code_stack.push(Atom::noop());
        code_atom(&mut test_state);
        assert!(
            test_state.bool_stack.last_eq(&true),
            "Should push true for Instruction"
        );
        test_state = PushState::new();
        test_state.code_stack.push(Atom::empty_list());
        code_atom(&mut test_state);
        assert!(
            test_state.bool_stack.last_eq(&false),
            "Should push false for Code Block"
        );
    }

    #[test]
    fn code_first_pushes_first_element_when_cb_is_found() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2), Atom::int(3)]));
        code_first(&mut test_state);
        assert_eq!(test_state.code_stack.to_string(), "1:Literal(3);");
    }

    #[test]
    fn code_rest_pushes_all_except_first_element() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2), Atom::int(3)]));
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:List: 1:Literal(3); 2:Literal(2); 3:Literal(1);;"
        );
        code_rest(&mut test_state);
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:List: 1:Literal(2); 2:Literal(1);;"
        );
    }

    #[test]
    fn code_cons_appends_in_reverse_order() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(1));
        test_state.code_stack.push(Atom::int(2));
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:Literal(2); 2:Literal(1);"
        );
        code_cons(&mut test_state);
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:List: 1:Literal(1); 2:Literal(2);;"
        );
    }

    #[test]
    fn code_container_finds_subelement() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2)]));
        test_state.code_stack.push(Atom::list(vec![
            Atom::list(vec![
                Atom::int(3),
                Atom::list(vec![Atom::int(1), Atom::int(2)]),
                Atom::list(vec![Atom::int(3), Atom::int(3)]),
                Atom::int(3),
            ]),
            Atom::int(4),
            Atom::int(5),
        ]));
        code_container(&mut test_state);
        // The top element is expected to be the smallest container of (1 2)' => (3 (1 2)' (3 3)' 3)'
        assert!(test_state
            .code_stack
            .to_string()
            .starts_with("1:List: 1:Literal(3); 2:List: 1:Literal(3); 2:Literal(3);; 3:List: 1:Literal(2); 2:Literal(1);; 4:Literal(3)"));
    }

    #[test]
    fn code_contains_pushes_true_if_second_contains_first() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2)]));
        test_state.code_stack.push(Atom::list(vec![
            Atom::list(vec![
                Atom::int(3),
                Atom::list(vec![Atom::int(1), Atom::int(2)]),
                Atom::list(vec![Atom::int(3), Atom::int(3)]),
                Atom::int(3),
            ]),
            Atom::int(4),
            Atom::int(5),
        ]));
        code_contains(&mut test_state);
        assert_eq!(test_state.bool_stack.to_string(), "1:true;");
    }

    #[test]
    fn code_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(2));
        test_state.name_stack.push(&"TEST");
        code_define(&mut test_state);
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Atom::int(2).to_string()
        );
    }

    #[test]
    fn code_definition_pushes_to_code_stack() {
        let mut test_state = PushState::new();
        test_state.name_bindings.insert("TEST", Atom::int(2));
        test_state.name_stack.push("TEST");
        code_definition(&mut test_state);
        assert_eq!(
            test_state.code_stack.pop().unwrap().to_string(),
            Atom::int(2).to_string()
        );
    }

    #[test]
    fn code_discrepancy_calculates_zero_discrepancy_correctly() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2)]));
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2)]));
        code_discrepancy(&mut test_state);
        assert_eq!(test_state.int_stack.to_string(), "1:0;");
    }

    #[test]
    fn code_discrepancy_calculates_discrepancy_correctly() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(0), Atom::int(2)]));
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2)]));
        code_discrepancy(&mut test_state);
        assert_eq!(test_state.int_stack.to_string(), "1:1;");
    }

    #[test]
    fn code_do_adds_instruction_to_excecution_stack() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(9));
        code_do(&mut test_state);
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1:Literal(9); 2:InstructionMeta(CODE.POP);"
        );
    }

    #[test]
    fn code_pop_and_do_adds_instruction_to_excecution_stack() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::int(9));
        code_pop_and_do(&mut test_state);
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1:InstructionMeta(CODE.POP); 2:Literal(9);"
        );
    }

    #[test]
    fn code_do_range_counts_upwards() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::noop());
        test_state.int_stack.push(3); // Current index
        test_state.int_stack.push(5); // Destination index
        code_do_range(&mut test_state);
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1:InstructionMeta(NOOP); 2:InstructionMeta(CODE.DO*RANGE);"
        );
        assert_eq!(test_state.int_stack.to_string(), "1:5; 2:4;");
    }

    #[test]
    fn code_do_range_counts_downwards() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::noop());
        test_state.int_stack.push(6); // Current index
        test_state.int_stack.push(1); // Destination index
        code_do_range(&mut test_state);
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1:InstructionMeta(NOOP); 2:InstructionMeta(CODE.DO*RANGE);"
        );
        assert_eq!(test_state.int_stack.to_string(), "1:1; 2:5;");
    }

    #[test]
    fn code_dup_duplicates_top_element() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::noop());
        code_dup(&mut test_state);
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:InstructionMeta(NOOP); 2:InstructionMeta(NOOP);"
        );
    }

    #[test]
    fn code_flush_empties_stack() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(0), Atom::int(2)]));
        test_state
            .code_stack
            .push(Atom::list(vec![Atom::int(1), Atom::int(2)]));
        code_flush(&mut test_state);
        assert_eq!(test_state.int_stack.to_string(), "");
    }

    #[test]
    fn code_from_bool_pushes_literal() {
        let mut test_state = PushState::new();
        // Test element is (1 2)'
        test_state.bool_stack.push(true);
        code_from_bool(&mut test_state);
        assert_eq!(test_state.code_stack.to_string(), "1:Literal(true);");
    }
}
