fn main() {
    let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";
    let mut tokens: Vec<&str> = input.split_whitespace().collect();
    println!("tokens = {:?}", tokens);

    let mut float_stack: Vec<f32> = Vec::new();
    let mut exec_stack: Vec<&str> = Vec::new();
    let mut code_stack: Vec<&str> = Vec::new();
    let mut int_stack: Vec<i32> = Vec::new();
    let mut bool_stack: Vec<bool> = Vec::new();

    // Push P onto the EXEC stack
    code_stack.extend(&tokens);
    // LOOP until the EXEC stack is empty:
    exec_stack.extend(&tokens);

    loop {
        // TODO: Stop conditions here

        let token = match tokens.pop() {
            None => break,
            Some(token) => token,
        };
        // Check Atom type
        let atom = match parse_atom(token) {
            None => break,
            Some(Literal) => break,
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

fn parse_atom(atom: &str) -> Option<Atom> {
    None
}

enum Atom {
    CodeBlock,
    Closer,
    Literal(Literal),
    InstructionMeta,
    Input,
}

struct Literal {}
