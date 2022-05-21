use std::env;

use pushr::push::instructions::InstructionSet;
use pushr::push::interpreter::PushInterpreter;
use pushr::push::parser::PushParser;
use pushr::push::state::PushState;
use pushr::push::item::Item;

fn main() {
    println!("Hello from Pushr");

    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        println!("No input ... Done");
        return;
    }
    let input = &args[1]; 

    let mut push_state = PushState::new();
    let mut instruction_set = InstructionSet::new();
    let instruction_cache = instruction_set.cache();

    instruction_set.load();
    PushParser::parse_program(&mut push_state, &instruction_set, &input);

    // Inject interpreter binary 
    push_state.name_bindings.insert("BIN".to_string(), Item::id(args[0].clone())); 

    loop {
        if PushInterpreter::step(&mut push_state, &mut instruction_set, &instruction_cache) {
            break;
       }
    }
    println!("STATE \n{}", push_state.to_string());
    println!("Done.");
}
