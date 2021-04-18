mod push;

use self::push::atoms::{Atom, InstructionMeta, Literal, PushType};

fn main() {
    let input = "( 2 3 INTEGER.* 4.1 5.2 FLOAT.+ TRUE FALSE BOOLEAN.OR )";
    let mut tokens: Vec<&str> = input.split_whitespace().collect();
    println!("tokens = {:?}", tokens);

    // Push P onto the EXEC stack

    loop {
        // TODO: Stop conditions here

        let token = match tokens.pop() {
            None => break,
            Some(token) => token,
        };
        // Check Atom type
        match parse_atom(token) {
            None => break,
            Some(Atom::Literal(atom)) => match atom.pushType {
                PushType::PushBoolType { val } => println!("Push bool {}", val),
                PushType::PushIntType { val } => println!("Push int {}", val),
                PushType::PushFloatType { val } => println!("Push float {}", val),
            },
            Some(Atom::InstructionMeta(atom)) => continue,

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

fn parse_atom(atom: &str) -> Option<Atom> {
    // TODO: Define Instruction Set
    None
}
