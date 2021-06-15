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
        push_state.code_stack.push(Atom::CodeBlock {
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
    if push_state.code_stack.last_eq(&Atom::block()) {
        match push_state.code_stack.pop() {
            Some(Atom::CodeBlock { mut atoms }) => {
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
        Some(Atom::CodeBlock { mut atoms }) => {
            atoms.pop();
            push_state.code_stack.push(Atom::CodeBlock { atoms: atoms });
        }
        _ => (),
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
            test_state.code_stack.last_eq(&Atom::block()),
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
        test_state.code_stack.push(Atom::block());
        code_atom(&mut test_state);
        assert!(
            test_state.bool_stack.last_eq(&false),
            "Should push false for Code Block"
        );
    }

    #[test]
    fn code_first_pushes_first_element_when_cb_is_found() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::CodeBlock {
            atoms: PushStack::from_vec(vec![Atom::int(1), Atom::int(2), Atom::int(3)]),
        });
        code_first(&mut test_state);
        assert_eq!(test_state.code_stack.to_string(), "1:Literal(3);");
    }

    #[test]
    fn code_rest_pushes_all_except_first_element() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::CodeBlock {
            atoms: PushStack::from_vec(vec![Atom::int(1), Atom::int(2), Atom::int(3)]),
        });
        code_rest(&mut test_state);
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:CodeBlock: 1:Literal(2); 2:Literal(1);;"
        );
    }
}
