use crate::push::atoms::Atom;
use crate::push::instructions::Instruction;
use crate::push::stack::PushStack;
use crate::push::state::PushState;
use std::collections::HashMap;

/// Maps the default code instructions to their identifiers
pub fn load_code_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("CODE.="), Instruction::new(code_eq, 0));
    map.insert(
        String::from("CODE.APPEND"),
        Instruction::new(code_append, 0),
    );
    map.insert(String::from("CODE.ATOM"), Instruction::new(code_atom, 0));
    map.insert(String::from("CODE.CAR"), Instruction::new(code_first, 0));
    map.insert(String::from("CODE.CDR"), Instruction::new(code_rest, 0));
    map.insert(String::from("CODE.CONS"), Instruction::new(code_cons, 0));
    map.insert(
        String::from("CODE.CONTAINER"),
        Instruction::new(code_container, 0),
    );
    map.insert(
        String::from("CODE.CONTAINS"),
        Instruction::new(code_contains, 0),
    );
    map.insert(
        String::from("CODE.DEFINE"),
        Instruction::new(code_define, 0),
    );
    map.insert(
        String::from("CODE.DEFINITION"),
        Instruction::new(code_definition, 0),
    );
    map.insert(
        String::from("CODE.DISCREPANCY"),
        Instruction::new(code_discrepancy, 0),
    );
    map.insert(
        String::from("CODE.DO"),
        Instruction::new(code_discrepancy, 0),
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
        push_state.exec_stack.push(Atom::InstructionMeta {
            name: "CODE.POP",
            code_blocks: 0,
        });
        push_state.exec_stack.push(instruction);
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
}
