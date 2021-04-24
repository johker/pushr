use crate::push::atoms::{Atom, InstructionMeta};
use crate::push::instructions::InstructionSet;
use crate::push::state::PushState;

pub struct PushInterpreter {
    instruction_set: InstructionSet,
    push_state: PushState,
}

impl PushInterpreter {
    fn parse_program(&mut self, code: &str) {
        for token in code.split_whitespace().rev() {
            println!("token = {:?}", token);
            if ")" == token {
                continue;
            }
            match self.instruction_set.map.get(token) {
                Some(instruction) => {
                    self.push_state.exec_stack.push(Atom::InstructionMeta {
                        name: token.to_string(),
                        code_blocks: 0,
                    });
                    continue;
                }
                None => (),
            }
        }

        match is.elements.get(atom) {
            Some(instruction) => return Atom::InstructionMeta(),
            None => println!("{} is unreviewed.", atom),
        }

        match atom.to_string().parse::<i32>() {
            Ok(val) => println!("Integer found: {}", val),
            Err(why) => println!("Doesnt look like a number: {}", why),
        }
        None
    }
}
