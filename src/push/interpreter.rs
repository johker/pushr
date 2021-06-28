use crate::push::atoms::{Atom, PushType};
use crate::push::instructions::InstructionSet;
use crate::push::parser::PushParser;
use crate::push::stack::PushStack;
use crate::push::state::PushState;

pub struct PushInterpreter<'a> {
    pub instruction_set: &'a mut InstructionSet,
    pub push_state: &'a mut PushState<'a>,
}

impl<'a> PushInterpreter<'a> {
    pub fn new(instruction_set: &'a mut InstructionSet, push_state: &'a mut PushState<'a>) -> Self {
        Self {
            instruction_set: instruction_set,
            push_state: push_state,
        }
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn run_stack(&mut self, stack: &mut PushStack<Atom<'a>>) {
        loop {
            match stack.pop() {
                None => break,
                Some(Atom::Literal { push_type }) => match push_type {
                    PushType::PushBoolType { val } => self.push_state.bool_stack.push(val),
                    PushType::PushIntType { val } => self.push_state.int_stack.push(val),
                    PushType::PushFloatType { val } => self.push_state.float_stack.push(val),
                },
                Some(Atom::InstructionMeta {
                    name,
                    code_blocks: _,
                }) => {
                    if let Some(instruction) = self.instruction_set.map.get_mut(name) {
                        (instruction.execute)(&mut self.push_state);
                    }
                }
                Some(Atom::List { mut atoms }) => {
                    // TODO: Push to exec stack in reverse order
                    self.run_stack(&mut atoms);
                }
                // TODO
                Some(Atom::Closer) => continue,
                Some(Atom::Input) => continue,
            }
        }
    }

    pub fn run(&mut self) {
        self.copy_to_code_stack();
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
                Some(Atom::InstructionMeta {
                    name,
                    code_blocks: _,
                }) => {
                    if let Some(instruction) = self.instruction_set.map.get_mut(name) {
                        (instruction.execute)(&mut self.push_state);
                    }
                }
                Some(Atom::List { atoms: _ }) => {
                    // TODO: Push to exec stack in reverse order
                }

                // TODO
                Some(Atom::Closer) => continue,
                Some(Atom::Input) => continue,
            };
            // TODO: Growth cap here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn copy_simple_program_to_code_stack() {
        let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&instruction_set, &mut push_state, &input);
        let mut interpreter = PushInterpreter::new(&mut instruction_set, &mut push_state);
        interpreter.copy_to_code_stack();
        assert_eq!(interpreter.push_state.code_stack.to_string(), "1:List: 1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1); 5:Literal(5.2); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);; 2:Closer;");
    }

    #[test]
    pub fn run_simple_program() {
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();

        push_state.exec_stack.push(Atom::InstructionMeta {
            name: "BOOLEAN.OR",
            code_blocks: 0,
        });
        push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushBoolType { val: false },
        });
        push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushBoolType { val: true },
        });

        push_state.exec_stack.push(Atom::InstructionMeta {
            name: "FLOAT.+",
            code_blocks: 0,
        });
        push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushFloatType { val: 5.2 },
        });
        push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushFloatType { val: 4.1 },
        });

        push_state.exec_stack.push(Atom::InstructionMeta {
            name: "INTEGER.*",
            code_blocks: 0,
        });
        push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushIntType { val: 3 },
        });
        push_state.exec_stack.push(Atom::Literal {
            push_type: PushType::PushIntType { val: 2 },
        });
        let mut interpreter = PushInterpreter::new(&mut instruction_set, &mut push_state);
        assert_eq!(interpreter.push_state.exec_stack.to_string(), "1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1); 5:Literal(5.2); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);");

        interpreter.run();
        assert_eq!(interpreter.push_state.int_stack.to_string(), "1:6;");
        assert!(
            (interpreter.push_state.float_stack.observe_vec(1).unwrap()[0] - 9.3).abs() < 0.00001
        );
        assert_eq!(interpreter.push_state.bool_stack.to_string(), "1:true;");
    }
}
