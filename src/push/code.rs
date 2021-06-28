use crate::push::atoms::Atom;
use crate::push::instructions::Instruction;
use crate::push::stack::PushStack;
use crate::push::state::PushState;
use std::collections::HashMap;

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
}

//
// ------------------ Type: BOOLEAN ---------------------
//

pub fn code_eq(push_state: &mut PushState) {
    if let Some(pv) = push_state.code_stack.observe_vec(2) {
        push_state
            .bool_stack
            .push(pv[0].to_string() == pv[1].to_string());
    }
}

pub fn code_append(push_state: &mut PushState) {
    if let Some(pv) = push_state.code_stack.pop_vec(2) {
        push_state.code_stack.push(Atom::List {
            atoms: PushStack::from_vec(pv),
        });
    }
}

pub fn code_atom(push_state: &mut PushState) {
    // Equality only checks type and ignores value
    push_state.bool_stack.push(
        push_state.code_stack.last_eq(&Atom::int(0))
            || push_state.code_stack.last_eq(&Atom::noop()),
    );
}

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
                        container = sublist.observe(sublist.size() - (*x as usize)).unwrap();
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
}
