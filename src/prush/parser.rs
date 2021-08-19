use crate::prush::instructions::InstructionSet;
use crate::prush::item::Item;
use crate::prush::stack::PushStack;
use crate::prush::state::PushState;
use crate::prush::vector::{BoolVector, FloatVector, IntVector};

pub struct PushParser {}

#[derive(Debug)]
pub enum VectorType {
    Bool,
    Int,
    Float,
}

impl<'a> PushParser {
    /// Recursivley performs a front push to the stack. It keeps track of the open sublist by a depth
    /// parameter. Returns true if the operation was sucessful
    pub fn rec_push(stack: &mut PushStack<Item>, item: Item, depth: usize) -> bool {
        if depth == 0 {
            // Push at this level
            stack.push_front(item);
            return true;
        }
        if let Some(mut bottom_item) = stack.bottom_mut() {
            match &mut bottom_item {
                Item::List { items } => {
                    // If the bottm element is a List push to its stack
                    return PushParser::rec_push(items, item, depth - 1);
                }
                _ => {
                    // Error: No more list found but depth > 0
                    false
                }
            }
        } else {
            // Empty stack -> just push
            stack.push(item);
            true
        }
    }

    /// Determines vector type and pushes corresponding item to stack. Ignores
    /// token if elements are not consistent.
    pub fn parse_vector(
        push_state: &mut PushState,
        depth: usize,
        vector_type: &VectorType,
        vector_token: &'a str,
    ) {
        match vector_type {
            VectorType::Bool => {
                let mut bv = vec![];
                for el in vector_token.split(",") {
                    if "1" == el || "true" == el {
                        bv.push(true);
                    } else if "0" == el || "false" == el {
                        bv.push(false);
                    } else {
                        return;
                    }
                }
                PushParser::rec_push(
                    &mut push_state.exec_stack,
                    Item::boolvec(BoolVector::new(bv)),
                    depth,
                );
            }
            VectorType::Int => {
                let mut iv = vec![];
                for el in vector_token.split(",") {
                    match el.to_string().parse::<i32>() {
                        Ok(ival) => iv.push(ival),
                        Err(_) => return,
                    }
                }
                PushParser::rec_push(
                    &mut push_state.exec_stack,
                    Item::intvec(IntVector::new(iv)),
                    depth,
                );
            }
            VectorType::Float => {
                let mut fv = vec![];
                for el in vector_token.split(",") {
                    match el.to_string().parse::<f32>() {
                        Ok(fval) => fv.push(fval),
                        Err(_) => return,
                    }
                }
                PushParser::rec_push(
                    &mut push_state.exec_stack,
                    Item::floatvec(FloatVector::new(fv)),
                    depth,
                );
            }
        }
    }

    /// Splits a string into tokens and front pushes it to the stack s.t. the
    /// end of the string ends up at the top of the stack.
    pub fn parse_program(
        push_state: &mut PushState,
        instruction_set: &InstructionSet,
        code: &'a str,
    ) {
        let mut depth = 0;
        for token in code.split_whitespace() {
            if token.starts_with("INT[") {
                PushParser::parse_vector(
                    push_state,
                    depth,
                    &VectorType::Int,
                    &token[4..token.len() - 1],
                );
                continue;
            }
            if token.starts_with("FLOAT[") {
                PushParser::parse_vector(
                    push_state,
                    depth,
                    &VectorType::Float,
                    &token[6..token.len() - 1],
                );
                continue;
            }
            if token.starts_with("BOOL[") {
                PushParser::parse_vector(
                    push_state,
                    depth,
                    &VectorType::Bool,
                    &token[5..token.len() - 1],
                );
                continue;
            }
            if "(" == token {
                PushParser::rec_push(
                    &mut push_state.exec_stack,
                    Item::List {
                        items: PushStack::new(),
                    },
                    depth,
                );
                // Start of (sub) list
                depth += 1;
                continue;
            }
            if ")" == token {
                // End of (sub) list
                depth -= 1;
                continue;
            }

            // Check for instruction
            if instruction_set.is_instruction(token) {
                PushParser::rec_push(
                    &mut push_state.exec_stack,
                    Item::instruction(token.to_string()),
                    depth,
                );
                continue;
            }
            // Check for Literal
            match token.to_string().parse::<i32>() {
                Ok(ival) => {
                    PushParser::rec_push(&mut push_state.exec_stack, Item::int(ival), depth);
                    continue;
                }
                Err(_) => (),
            }
            match token.to_string().parse::<f32>() {
                Ok(fval) => {
                    PushParser::rec_push(&mut push_state.exec_stack, Item::float(fval), depth);
                    continue;
                }
                Err(_) => (),
            }

            match token {
                "TRUE" => {
                    PushParser::rec_push(&mut push_state.exec_stack, Item::bool(true), depth);
                    continue;
                }
                "FALSE" => {
                    PushParser::rec_push(&mut push_state.exec_stack, Item::bool(false), depth);
                    continue;
                }
                &_ => {
                    PushParser::rec_push(
                        &mut push_state.exec_stack,
                        Item::name(token.to_string()),
                        depth,
                    );
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
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        assert_eq!(push_state.exec_stack.to_string(), "1:List: 1:Literal(2); 2:Literal(3); 3:InstructionMeta(INTEGER.*); 4:Literal(4.1f); 5:Literal(5.2f); 6:InstructionMeta(FLOAT.+); 7:Literal(true); 8:Literal(false); 9:InstructionMeta(BOOLEAN.OR);;")
    }

    #[test]
    pub fn parse_potentiation_program() {
        let input = "( ARG FLOAT.DEFINE EXEC.Y ( ARG FLOAT.* 1 INTEGER.- INTEGER.DUP 0 INTEGER.> EXEC.IF ( ) EXEC.POP ) ) ";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        assert_eq!(
            push_state.exec_stack.to_string(),
            "1:List: 1:Identifier(ARG); 2:InstructionMeta(FLOAT.DEFINE); 3:InstructionMeta(EXEC.Y); 4:List: 1:Identifier(ARG); 2:InstructionMeta(FLOAT.*); 3:Literal(1); 4:InstructionMeta(INTEGER.-); 5:InstructionMeta(INTEGER.DUP); 6:Literal(0); 7:InstructionMeta(INTEGER.>); 8:InstructionMeta(EXEC.IF); 9:List: ; 10:InstructionMeta(EXEC.POP);;;"
        );
    }

    #[test]
    pub fn parse_factorial_program() {
        let input = "( CODE.QUOTE ( CODE.DUP INTEGER.DUP 1 INTEGER.- CODE.DO INTEGER.* )
                       CODE.QUOTE ( INTEGER.POP 1 )
                                      INTEGER.DUP 2 INTEGER.< CODE.IF )";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        assert_eq!(
            push_state.exec_stack.to_string(),
            "1:List: 1:InstructionMeta(CODE.QUOTE); 2:List: 1:InstructionMeta(CODE.DUP); 2:InstructionMeta(INTEGER.DUP); 3:Literal(1); 4:InstructionMeta(INTEGER.-); 5:InstructionMeta(CODE.DO); 6:InstructionMeta(INTEGER.*);; 3:InstructionMeta(CODE.QUOTE); 4:List: 1:InstructionMeta(INTEGER.POP); 2:Literal(1);; 5:InstructionMeta(INTEGER.DUP); 6:Literal(2); 7:InstructionMeta(INTEGER.<); 8:InstructionMeta(CODE.IF);;");
    }

    #[test]
    pub fn parse_different_vector_types_with_correct_syntax() {
        let input = "( BOOL[1,1,1,0,0] INT[2,345,-5] FLOAT[3.3,1.2,4.1] )";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        assert_eq!(
            push_state.exec_stack.to_string(),
            "1:List: 1:Literal([1,1,1,0,0]); 2:Literal([2,345,-5]); 3:Literal([3.3,1.2,4.1]);;"
        );
    }

    #[test]
    pub fn parse_different_vector_types_with_wrong_syntax() {
        let input = "( BOOL[1,1,2,0,0] INT[2,345,-5.0] FLOAT[3.3,NAN,4.1] INT[1,2,3] )";
        let mut push_state = PushState::new();
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        PushParser::parse_program(&mut push_state, &instruction_set, &input);
        assert_eq!(
            push_state.exec_stack.to_string(),
            "1:List: 1:Literal([1,2,3]);;"
        );
    }
}
