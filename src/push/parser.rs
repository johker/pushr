use crate::push::atoms::{Atom, PushType};
use crate::push::instructions::InstructionSet;
use crate::push::interpreter::PushInterpreter;
use crate::push::stack::PushStack;
use crate::push::state::PushState;

pub struct PushParser {}

impl<'a> PushParser {
    pub fn rec_push(stack: &mut PushStack<Atom<'a>>, atom: Atom<'a>) -> bool {
        // Push recursively
        if let Some(mut first_el) = stack.bottom_mut() {
            match &mut first_el {
                Atom::List { atoms } => {
                    // If the top element is a List
                    // push to its stack
                    return PushParser::rec_push(atoms, atom);
                }
                _ => {
                    if atom == Atom::Closer {
                        // Closer is pushed on higher stack
                        // to mark end of List
                        false
                    } else {
                        // Push any other element to top
                        stack.push_front(atom);
                        true
                    }
                }
            }
        } else {
            // Empty stack -> just push
            stack.push(atom);
            true
        }
    }

    pub fn parse_program(
        instruction_set: &InstructionSet,
        push_state: &mut PushState<'a>,
        code: &'a str,
    ) {
        for token in code.split_whitespace() {
            if "(" == token {
                // Start of code block
                PushParser::rec_push(
                    &mut push_state.exec_stack,
                    Atom::List {
                        atoms: PushStack::new(),
                    },
                );
                continue;
            }
            if ")" == token {
                // End of code block
                if !PushParser::rec_push(&mut push_state.exec_stack, Atom::Closer) {
                    push_state.exec_stack.push_front(Atom::Closer);
                }
                continue;
            }

            // Check for instruction
            match instruction_set.map.get(token) {
                Some(_instruction) => {
                    let im = Atom::InstructionMeta { name: token };
                    PushParser::rec_push(&mut push_state.exec_stack, im);
                    continue;
                }
                None => (),
            }
            // Check for Literal
            match token.to_string().parse::<i32>() {
                Ok(ival) => {
                    PushParser::rec_push(
                        &mut push_state.exec_stack,
                        Atom::Literal {
                            push_type: PushType::PushIntType { val: ival },
                        },
                    );
                    continue;
                }
                Err(_) => (),
            }
            match token.to_string().parse::<f32>() {
                Ok(fval) => {
                    PushParser::rec_push(
                        &mut push_state.exec_stack,
                        Atom::Literal {
                            push_type: PushType::PushFloatType { val: fval },
                        },
                    );
                    continue;
                }
                Err(_) => (),
            }
            match token {
                "TRUE" => {
                    PushParser::rec_push(
                        &mut push_state.exec_stack,
                        Atom::Literal {
                            push_type: PushType::PushBoolType { val: true },
                        },
                    );
                    continue;
                }
                "FALSE" => {
                    PushParser::rec_push(
                        &mut push_state.exec_stack,
                        Atom::Literal {
                            push_type: PushType::PushBoolType { val: false },
                        },
                    );
                    continue;
                }
                &_ => {
                    if let Some(instruction) = push_state.name_bindings.get(token) {
                        // Existing name binding -> Push to execution stack
                        PushParser::rec_push(&mut push_state.exec_stack, instruction.clone());
                    } else {
                        // Unknown identifier -> Push onto name stack
                        push_state.name_stack.push(token.clone());
                    }
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_simple_program() {
        let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&instruction_set, &mut push_state, &input);
        let interpreter = PushInterpreter::new(&mut instruction_set, &mut push_state);

        assert_eq!(interpreter.push_state.exec_stack.to_string(), "1:List: 1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1); 5:Literal(5.2); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);; 2:Closer;")
    }
}
