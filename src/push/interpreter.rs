use crate::push::instructions::InstructionSet;
use crate::push::state::PushState;

pub struct PushInterpreter {
    instructionSet: InstructionSet,
    pushState: PushState,
}
