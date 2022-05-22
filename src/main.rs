use std::env;

use pushr::push::instructions::InstructionSet;
use pushr::push::interpreter::PushInterpreter;
use pushr::push::parser::PushParser;
use pushr::push::state::PushState;
use pushr::push::item::Item;

fn main() {
    println!("> ------------------");
    println!(">      PUSHR        ");
    println!("> ------------------");

    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        println!("No input ... Done");
        return;
    }
    let input = &args[1]; 
    println!("Input = {}", input);

    let mut push_state = PushState::new();
    let mut instruction_set = InstructionSet::new();
    let instruction_cache = instruction_set.cache();

    // Load program
    instruction_set.load();
    PushParser::parse_program(&mut push_state, &instruction_set, &input);
    PushParser::copy_to_code_stack(&mut push_state);

    // Inject interpreter binary 
    push_state.name_bindings.insert("BIN".to_string(), Item::id(args[0].clone())); 

    loop {
        println!("> EXEC  : {}", push_state.exec_stack.to_string());
        println!("> CODE  : {}", push_state.code_stack.to_string());
        println!("> INT   : {}", push_state.int_stack.to_string());
        println!("> ------------ ");
        if PushInterpreter::step(&mut push_state, &mut instruction_set, &instruction_cache) {
            break;
       }
    }
    println!("Done.");
}
