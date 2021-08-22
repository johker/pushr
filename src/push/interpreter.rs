use crate::push::instructions::InstructionSet;
use crate::push::item::{Item, PushType};
use crate::push::state::PushState;

pub struct PushInterpreter {}

impl PushInterpreter {
    /// Copies execution stack to code stack
    pub fn copy_to_code_stack(push_state: &mut PushState) {
        // Push top-level program to code stack
        if let Some(code) = push_state.exec_stack.copy_vec(push_state.exec_stack.size()) {
            push_state.code_stack.push_vec(code);
        }
    }

    /// Copies execution stack to code stac and recursively runs execution stack
    pub fn run(push_state: &mut PushState, instruction_set: &mut InstructionSet) {
        // TODO: Make static / Call run_stack
        PushInterpreter::copy_to_code_stack(push_state);
        let icache = instruction_set.cache();
        loop {
            // TODO: Stop conditions here
            match push_state.exec_stack.pop() {
                None => break,
                Some(Item::Literal { push_type }) => match push_type {
                    PushType::Bool { val } => push_state.bool_stack.push(val),
                    PushType::Int { val } => push_state.int_stack.push(val),
                    PushType::Float { val } => push_state.float_stack.push(val),
                    PushType::BoolVector { val } => push_state.bool_vector_stack.push(val),
                    PushType::FloatVector { val } => push_state.float_vector_stack.push(val),
                    PushType::IntVector { val } => push_state.int_vector_stack.push(val),
                },
                Some(Item::Identifier { name }) => {
                    if push_state.quote_name {
                        push_state.name_stack.push(name);
                        push_state.quote_name = false;
                    } else {
                        if let Some(item) = push_state.name_bindings.get(&*name) {
                            // Evaluate item for this name in next iteration
                            push_state.exec_stack.push(item.clone());
                        } else {
                            push_state.name_stack.push(name);
                        }
                    }
                }
                Some(Item::InstructionMeta { name }) => {
                    if let Some(instruction) = instruction_set.get_instruction(&name) {
                        (instruction.execute)(push_state, &icache);
                    }
                }
                Some(Item::List { mut items }) => {
                    if let Some(pv) = items.pop_vec(items.size()) {
                        push_state.exec_stack.push_vec(pv);
                    }
                }
            }
            // TODO: Growth cap here
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::parser::PushParser;

    #[test]
    pub fn copy_simple_program_to_code_stack() {
        let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        PushInterpreter::copy_to_code_stack(&mut push_state);
        assert_eq!(push_state.code_stack.to_string(), "1:List: 1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1f); 5:Literal(5.2f); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);;");
    }

    #[test]
    pub fn run_simple_program() {
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();

        push_state
            .exec_stack
            .push(Item::instruction("BOOLEAN.OR".to_string()));
        push_state.exec_stack.push(Item::bool(false));
        push_state.exec_stack.push(Item::bool(true));

        push_state
            .exec_stack
            .push(Item::instruction("FLOAT.+".to_string()));
        push_state.exec_stack.push(Item::float(5.2));
        push_state.exec_stack.push(Item::float(4.1));

        push_state
            .exec_stack
            .push(Item::instruction("INTEGER.*".to_string()));
        push_state.exec_stack.push(Item::int(3));
        push_state.exec_stack.push(Item::int(2));
        assert_eq!(push_state.exec_stack.to_string(), "1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1f); 5:Literal(5.2f); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);");

        PushInterpreter::run(&mut push_state, &mut instruction_set);
        assert_eq!(push_state.int_stack.to_string(), "1:6;");
        assert!((push_state.float_stack.copy_vec(1).unwrap()[0] - 9.3).abs() < 0.00001);
        assert_eq!(push_state.bool_stack.to_string(), "1:true;");
    }

    #[test]
    pub fn run_potentiation_program() {
        let input = "( ARG FLOAT.DEFINE EXEC.Y ( ARG FLOAT.* 1 INTEGER.- INTEGER.DUP 0 INTEGER.> EXEC.IF ( ) EXEC.POP ) ) ";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        push_state.int_stack.push(4);
        push_state.float_stack.push(2.0);
        PushInterpreter::run(&mut push_state, &mut instruction_set);
        assert_eq!(push_state.float_stack.to_string(), "1:16;");
    }

    #[test]
    pub fn run_factorial_program() {
        let input = "( CODE.QUOTE ( CODE.DUP INTEGER.DUP 1 INTEGER.- CODE.DO INTEGER.* )
                       CODE.QUOTE ( INTEGER.POP 1 )
                                      INTEGER.DUP 2 INTEGER.< CODE.IF )";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        push_state.int_stack.push(4);
        PushInterpreter::run(&mut push_state, &mut instruction_set);
        assert_eq!(push_state.int_stack.to_string(), "1:24;");
    }
}
