## Prush

Prush is Rust based interpreter for Push programs.

## What is Push?

Push is a stack-based Turing-complete programming language that enables autoconstructive evolution in its programs.
More information can be found [here](http://faculty.hampshire.edu/lspector/push.html).

## Supported Stack Types

This implementation supports all Push3 instructions for the types desribed in the [Push 3.0 Programming Language Description](http://faculty.hampshire.edu/lspector/push3-description.html#Type):

* BOOLEAN
* CODE
* EXECUTION
* FLOAT
* INTEGER
* NAME

Additionally, it provides the vector types for boolean, float and integer:

* BOOLVECTOR
* FLOATVECTOR
* INTVECTOR

The default instructions for vector types are dup, equal, flush, shove, stackdepth, swap, yank and yankdup. 

## Usage

```rust
// Define Push program:
let input = "( CODE.QUOTE ( CODE.DUP INTEGER.DUP 1 INTEGER.- CODE.DO INTEGER.* )
               CODE.QUOTE ( INTEGER.POP 1 )
               INTEGER.DUP 2 INTEGER.< CODE.IF )";
// Define State and Instruction Set
let mut push_state = PushState::new();
let mut instruction_set = InstructionSet::new();
// Load default instructions
instruction_set.load();
// Add program to execution stack
PushParser::parse_program(&mut push_state, &instruction_set, &input);
// Put initial values
push_state.int_stack.push(4);
// Run the program
PushInterpreter::run(&mut push_state, &mut instruction_set);
```






