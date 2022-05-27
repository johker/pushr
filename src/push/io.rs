use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::state::PushState;
use crate::push::stack::PushPrint;
use crate::push::vector::{BoolVector,IntVector};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct PushMessage {
    pub header: IntVector,
    pub body: BoolVector,
}

impl PushMessage {
    pub fn new(header: IntVector, body: BoolVector) -> Self {
        Self { header: header, body: body}
    }
}

impl PushPrint for PushMessage {
   fn to_pstring(&self) -> String {
       self.to_string()
   }
}

impl fmt::Display for PushMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg_str = format!("{}&{}", self.header.to_string(), self.body.to_string());
        write!(f, "{}", msg_str)
    }
}

impl PartialEq for PushMessage {
    fn eq(&self, other: &Self) -> bool {
        self.header.values == other.header.values && self.body.values == other.body.values
    }
}

pub fn load_io_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(
        String::from("INPUT.AVAILABLE"),
        Instruction::new(input_available),
    );
    map.insert(String::from("INPUT.GET"), Instruction::new(input_get));
    map.insert(String::from("INPUT.NEXT"), Instruction::new(input_next));
    map.insert(String::from("INPUT.READ"), Instruction::new(input_read));
    map.insert(
        String::from("INPUT.STACKDEPTH"),
        Instruction::new(input_stack_depth),
    );

    map.insert(String::from("OUTPUT.FLUSH"), Instruction::new(output_flush));
    map.insert(String::from("OUTPUT.WRITE"), Instruction::new(output_write));
    map.insert(
        String::from("OUTPUT.STACKDEPTH"),
        Instruction::new(output_stack_depth),
    );
}

/////////////////////////////////////// INPUT //////////////////////////////////////////

/// INPUT.AVAILABLE: This instruction pushes true to the BOOLEAN stack if the input
/// stack is not empty and false otherwise.
pub fn input_available(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if push_state.input_stack.size() > 0 {
        push_state.bool_stack.push(true);
    } else {
        push_state.bool_stack.push(false);
    }
}

/// INPUT.FLUSH: Empties the INPUT stack.
pub fn input_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.input_stack.flush();
}

/// INPUT.GET: Pushes the nth bit of the first element of the FIFO queue to the
/// BOOLEAN stack. The index n is taken from the INTEGER stack.
pub fn input_get(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let input_size = push_state.input_stack.size();
        if input_size > 0 {
            if let Some(input) = push_state.input_stack.peek_oldest() {
                let list_index =
                    i32::max(i32::min(input.body.values.len() as i32 - 1, index), 0) as usize;
                push_state.bool_stack.push(input.body.values[list_index]);
            }
        }
    }
}

/// INPUT.NEXT: Removes the first element of the input FIFO queue.
pub fn input_next(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.input_stack.pop();
}

/// INPUT.READ: This instruction reads the input stack as a FIFO queue. If non empty
/// it pushes a copy of the bottom item to the BOOLVECTOR stack.
pub fn input_read(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    let input_size = push_state.input_stack.size();
    if input_size > 0 {
        if let Some(input) = push_state.input_stack.copy_oldest() {
            push_state.bool_vector_stack.push(input.body);
            push_state.int_vector_stack.push(input.header);
        }
    }
}

/// INPUT.STACKDEPTH: Pushes the stack depth onto the INTEGER stack.
pub fn input_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.input_stack.size() as i32);
}

/////////////////////////////////////// OUTPUT /////////////////////////////////////////

/// OUTPUT.FLUSH: Empties the OUTPUT stack.
pub fn output_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.output_stack.flush();
}

/// OUTPUT.STACKDEPTH: Pushes the stack depth onto the INTEGER stack.
pub fn output_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.output_stack.size() as i32);
}

/// OUTPUT.WRITE: Creates a messages from the top items of the INTVECTOR stack (header) and
/// the BOOLVECTOR stack (body) and pushes it to the OUTPUT stack.
pub fn output_write(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(body) = push_state.bool_vector_stack.pop() {
        if let Some(header) = push_state.int_vector_stack.pop() {
            push_state.output_stack.push(PushMessage::new(header, body));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::vector::BoolVector;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn input_available_pushes_boolean() {
        let mut test_state = PushState::new();
        input_available(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), false);
        test_state
            .input_stack
            .push(PushMessage::new(IntVector::new(vec![]), BoolVector::from_int_array(vec![0, 0, 0, 1])));
        input_available(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn input_get_pushes_input_bit() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(1);
        test_state
            .input_stack
            .push(PushMessage::new(IntVector::new(vec![]), BoolVector::from_int_array(vec![0, 1, 0, 1])));
        input_get(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn input_read_pushes_bottom_if_available() {
        let mut test_state = PushState::new();
        input_read(&mut test_state, &icache());

        let header = IntVector::new(vec![]);
        let body1 = BoolVector::from_int_array(vec![1, 1]);
        let body2 = BoolVector::from_int_array(vec![0, 0]);
        test_state.input_stack.push(PushMessage::new(header.clone(), body1));
        test_state.input_stack.push(PushMessage::new(header.clone(), body2));

        input_read(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_vector_stack.pop().unwrap(),
            BoolVector::from_int_array(vec![1, 1])
        );
    }

    #[test]
    fn output_write_pushes_top_item() {
        let mut test_state = PushState::new();

        let test_header = IntVector::new(vec![]);
        test_state.int_vector_stack.push(test_header.clone());
        test_state.int_vector_stack.push(test_header);
        let test_vec1 = BoolVector::from_int_array(vec![1, 1]);
        let test_vec2 = BoolVector::from_int_array(vec![0, 0]);
        test_state.bool_vector_stack.push(test_vec1);
        test_state.bool_vector_stack.push(test_vec2);

        output_write(&mut test_state, &icache());
        assert_eq!(
            test_state.output_stack.pop().unwrap(),
            PushMessage::new(IntVector::new(vec![]), BoolVector::from_int_array(vec![0, 0]))
        );
    }
}
