use crate::push::atoms::{Atom, PushType};
use crate::push::instructions::InstructionSet;
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

    pub fn run(&mut self) {
        // TODO: Push to code stack here?
        // self.push_state.exec_stack.observe_vec();

        loop {
            // TODO: Stop conditions here

            let token = match self.push_state.exec_stack.pop() {
                None => break,
                Some(Atom::Literal { push_type }) => match push_type {
                    PushType::PushBoolType { val } => self.push_state.bool_stack.push(val),
                    PushType::PushIntType { val } => self.push_state.int_stack.push(val),
                    PushType::PushFloatType { val } => self.push_state.float_stack.push(val),
                },
                Some(Atom::InstructionMeta { name, code_blocks }) => {}

                // TODO
                Some(Atom::Closer) => continue,
                Some(Atom::CodeBlock) => continue,
                Some(Atom::Input) => continue,
            };
            // TODO: Growth cap here
        }

        // If the first item on the EXEC stack is a single instruction
        // then pop it and execute it.
        // Else if the first item on the EXEC stack is a literal
        // then pop it and push it onto the appropriate stack.
        // Else (the first item must be a list) pop it and push all of the
        // items that it contains back onto the EXEC stack individually,
        // in reverse order (so that the item that was first in the list ends up on top).
    }

    pub fn parse_program(&mut self, code: &'a str) {
        for token in code.split_whitespace().rev() {
            println!("token = {:?}", token);
            if ")" == token {
                continue;
            }
            // Check for instruction
            match self.instruction_set.map.get(token) {
                Some(instruction) => {
                    let x = Atom::InstructionMeta {
                        name: token,
                        code_blocks: instruction.code_blocks,
                    };
                    self.push_state.exec_stack.push(x);
                    continue;
                }
                None => (),
            }
            // Check for Literal
            match token.to_string().parse::<i32>() {
                Ok(ival) => {
                    self.push_state.exec_stack.push(Atom::Literal {
                        push_type: PushType::PushIntType { val: ival },
                    });
                    continue;
                }
                Err(why) => (),
            }
            match token.to_string().parse::<f32>() {
                Ok(fval) => {
                    self.push_state.exec_stack.push(Atom::Literal {
                        push_type: PushType::PushFloatType { val: fval },
                    });
                    continue;
                }
                Err(why) => (),
            }
            match token.to_string().parse::<bool>() {
                Ok(bval) => {
                    self.push_state.exec_stack.push(Atom::Literal {
                        push_type: PushType::PushBoolType { val: bval },
                    });
                    continue;
                }
                Err(why) => (),
            }
        }
    }
}
