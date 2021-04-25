mod push;

use self::push::atoms::{Atom, PushType};
use self::push::instructions::InstructionSet;
use self::push::state::PushState;

fn main() {
    let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";

    let push_state = PushState::new();
    let mut instruction_set = InstructionSet::new();
    instruction_set.load();
    // Push P onto the CODE stack
    // Push P onto the EXEC stack
}
