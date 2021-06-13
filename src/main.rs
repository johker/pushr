mod push;

use self::push::instructions::InstructionSet;
use self::push::interpreter::PushInterpreter;
use self::push::parser::PushParser;
use self::push::state::PushState;

fn main() {
    let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";

    let mut push_state = PushState::new();
    let mut instruction_set = InstructionSet::new();
    instruction_set.load();

    PushParser::parse_program(&instruction_set, &mut push_state, &input);
    let _interpreter = PushInterpreter::new(&mut instruction_set, &mut push_state);

    // Push P onto the EXEC stack
}
