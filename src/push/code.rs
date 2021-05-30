use crate::push::atoms::{Atom, PushType};
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
        push_state.code_stack.last_eq(&Atom::Literal {
            push_type: PushType::PushIntType { val: 0 },
        }) || push_state.code_stack.last_eq(&Atom::InstructionMeta {
            name: "",
            code_blocks: 0,
        }),
    );
}

pub fn code_car(push_state: &mut PushState) {
    if push_state.code_stack.last_eq(&Atom::CodeBlock {
        atoms: PushStack::new(),
    }) {
        match push_state.code_stack.pop() {
            Some(Atom::CodeBlock { mut atoms }) => {
                if let Some(atom) = atoms.pop() {
                    push_state.exec_stack.push(atom);
                }
                push_state.code_stack.push(Atom::CodeBlock { atoms: atoms });
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_car_pushes_to_exec_stack() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Atom::CodeBlock {
            atoms: PushStack::from_vec(vec![
                Atom::Literal {
                    push_type: PushType::PushIntType { val: 1 },
                },
                Atom::Literal {
                    push_type: PushType::PushIntType { val: 2 },
                },
            ]),
        });
        code_car(&mut test_state);
        assert_eq!(test_state.code_stack.size(), 1);
        assert_eq!(test_state.exec_stack.size(), 1);
    }
}
