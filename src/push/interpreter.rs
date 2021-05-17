use crate::push::atoms::{Atom, PushType};
use crate::push::instructions::InstructionSet;
use crate::push::stack::PushStack;
use crate::push::state::PushState;

pub struct PushInterpreter<'a> {
    instruction_set: InstructionSet,
    push_state: PushState<'a>,
}

impl<'a> PushInterpreter<'a> {
    pub fn new(instruction_set: InstructionSet, push_state: PushState<'a>) -> Self {
        Self {
            instruction_set: instruction_set,
            push_state: push_state,
        }
    }

    pub fn copy_to_code_stack(&mut self) {
        // Push top-level program to code stack
        if let Some(code) = self
            .push_state
            .exec_stack
            .observe_vec(self.push_state.exec_stack.size())
        {
            self.push_state.code_stack.push_vec(code);
        }
    }

    pub fn run(&mut self) {
        loop {
            // TODO: Stop conditions here
            // If the first item on the EXEC stack is a single instruction
            // then pop it and execute it.
            // Else if the first item on the EXEC stack is a literal
            // then pop it and push it onto the appropriate stack.
            // Else (the first item must be a list) pop it and push all of the
            // items that it contains back onto the EXEC stack individually,
            // in reverse order (so that the item that was first in the list ends up on top).

            match self.push_state.exec_stack.pop() {
                None => break,
                Some(Atom::Literal { push_type }) => match push_type {
                    PushType::PushBoolType { val } => self.push_state.bool_stack.push(val),
                    PushType::PushIntType { val } => self.push_state.int_stack.push(val),
                    PushType::PushFloatType { val } => self.push_state.float_stack.push(val),
                },
                Some(Atom::InstructionMeta { name, code_blocks }) => {
                    if let Some(instruction) = self.instruction_set.map.get_mut(name) {
                        (instruction.execute)(&mut self.push_state);
                    }
                }
                Some(Atom::CodeBlock { atoms }) => {
                    // TODO: Push to exec stack in reverse order
                }

                // TODO
                Some(Atom::Closer) => continue,
                Some(Atom::Input) => continue,
            };
            // TODO: Growth cap here
        }
    }

    pub fn push_atom(&mut self, atom: Atom<'a>) {
        // TODO Check recursively for CodeBlocks
        if let Some(mut first_el) = self.push_state.exec_stack.first_mut() {
        } else {
            // Empty stack -> just push
            self.push_state.exec_stack.push(atom);
        }
    }

    pub fn rec_push(stack: &mut PushStack<Atom<'a>>, atom: Atom<'a>) -> bool {
        // Push recursively
        if let Some(mut first_el) = stack.first_mut() {
            match &mut first_el {
                Atom::CodeBlock { atoms } => {
                    // If the top element is a CodeBlock
                    // push to its stack
                    return PushInterpreter::rec_push(atoms, atom);
                }
                _ => {
                    if atom == Atom::Closer {
                        // Closer is pushed on higher stack
                        // to mark end of CodeBlock
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

    pub fn parse_program(&mut self, code: &'a str) {
        for token in code.split_whitespace() {
            if "(" == token {
                // Start of code block
                PushInterpreter::rec_push(
                    &mut self.push_state.exec_stack,
                    Atom::CodeBlock {
                        atoms: PushStack::new(),
                    },
                );
                continue;
            }
            if ")" == token {
                // End of code block
                if !PushInterpreter::rec_push(&mut self.push_state.exec_stack, Atom::Closer) {
                    self.push_state.exec_stack.push_front(Atom::Closer);
                }
                continue;
            }

            // Check for instruction
            match self.instruction_set.map.get(token) {
                Some(instruction) => {
                    let im = Atom::InstructionMeta {
                        name: token,
                        code_blocks: instruction.code_blocks,
                    };
                    PushInterpreter::rec_push(&mut self.push_state.exec_stack, im);
                    continue;
                }
                None => (),
            }
            // Check for Literal
            match token.to_string().parse::<i32>() {
                Ok(ival) => {
                    PushInterpreter::rec_push(
                        &mut self.push_state.exec_stack,
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
                    PushInterpreter::rec_push(
                        &mut self.push_state.exec_stack,
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
                    PushInterpreter::rec_push(
                        &mut self.push_state.exec_stack,
                        Atom::Literal {
                            push_type: PushType::PushBoolType { val: true },
                        },
                    );
                    continue;
                }
                "FALSE" => {
                    PushInterpreter::rec_push(
                        &mut self.push_state.exec_stack,
                        Atom::Literal {
                            push_type: PushType::PushBoolType { val: false },
                        },
                    );
                    continue;
                }
                &_ => {
                    if let Some(instruction) = self.push_state.name_bindings.get(token) {
                        // Existing name binding -> Push to execution stack
                        PushInterpreter::rec_push(
                            &mut self.push_state.exec_stack,
                            instruction.clone(),
                        );
                    } else {
                        // Unknown identifier -> Push onto name stack
                        self.push_state.name_stack.push(token);
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
        let push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        let mut interpreter = PushInterpreter::new(instruction_set, push_state);

        interpreter.parse_program(&input);
        assert_eq!(interpreter.push_state.exec_stack.to_string(), "1:CodeBlock: 1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1); 5:Literal(5.2); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);; 2:Closer;")
    }

    #[test]
    pub fn run_simple_program() {
        let push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        let mut interpreter = PushInterpreter::new(instruction_set, push_state);

        interpreter
            .push_state
            .exec_stack
            .push(Atom::InstructionMeta {
                name: "BOOLEAN.OR",
                code_blocks: 0,
            });
        interpreter.push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushBoolType { val: false },
        });
        interpreter.push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushBoolType { val: true },
        });
        interpreter
            .push_state
            .exec_stack
            .push(Atom::InstructionMeta {
                name: "FLOAT.+",
                code_blocks: 0,
            });
        interpreter.push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushFloatType { val: 5.2 },
        });
        interpreter.push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushFloatType { val: 4.1 },
        });
        interpreter
            .push_state
            .exec_stack
            .push(Atom::InstructionMeta {
                name: "INTEGER.*",
                code_blocks: 0,
            });
        interpreter.push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushIntType { val: 3 },
        });
        interpreter.push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushIntType { val: 2 },
        });
        assert_eq!(interpreter.push_state.exec_stack.to_string(), "1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1); 5:Literal(5.2); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);");

        interpreter.run();
        assert_eq!(interpreter.push_state.int_stack.to_string(), "1:6;");
        assert!(
            (interpreter.push_state.float_stack.observe_vec(1).unwrap()[0] - 9.3).abs() < 0.00001
        );
        assert_eq!(interpreter.push_state.bool_stack.to_string(), "1:true;");
    }
}
