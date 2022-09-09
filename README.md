## Pushr

![example workflow](https://github.com/johker/pushr/actions/workflows/rust.yml/badge.svg)

Pushr is a Rust based interpreter for Push programs.

## What is Push?

Push is a stack-based, Turing-complete programming language that enables autoconstructive evolution in its programs.
More information can be found [here](http://faculty.hampshire.edu/lspector/push.html).

## Supported Stack Types

This implementation supports all Push3 instructions for the types desribed in the [Push 3.0 Programming Language Description](http://faculty.hampshire.edu/lspector/push3-description.html#Type):

* BOOLEAN
* CODE
* EXECUTION
* FLOAT
* INTEGER
* NAME

Additional stack types:

* BOOLVECTOR: vector with boolean elements
* FLOATVECTOR: vector with float elements
* INTVECTOR: vector with integer elements
* INDEX: simplifies loop syntax
* GRAPH: graph object used as a memory

FIFO queues are used to communicate with other modules. The type is BOOLVECTOR. 
* INPUT
* OUTPUT


## Supported instructions

The default instructions for vector types are 'dup', 'equal', 'flush', 'get', 'set', 'shove', 'stackdepth', 'rand', 'swap', 'yank' and 'yankdup'. Additionally, the instruction set contains 'add', 'subtract', 'multiply' and 'divide' for float and integer vectors, as well as 'and', 'or' and 'not' for boolean vectors. To initialize vectors the instructions 'ones'  and 'zeros' can be used.

For vector instructions the following rules apply: 

* The 'rand' instruction is interpreted differently for boolean, float and integer vectors: 
   - BOOLVECTOR.RAND randomly distributes (sparsity * n) 'true' values acrross an array of length n where sparsity is the percentage of active bits.
   - INTVECTOR.RAND draws n samples from the uniform distribution U(min,max).
   - FLOATVECTOR.RAND draws n samples form the normal distribution N(mu,sig).

* Vector lengths do not have to match. Arithmetic operations are executed element-wise on the overlapping parts. An offset parameter shifts the top vector on the stack to create the desired overlap. 

* In a Push program the vectors are defined as BOOL[..], FLOAT[..] and INT[..]. For example, BOOL[1,0] defines a BOOLVECTOR with two elements. 


## Usage

The following example shows how to intepret Push program with Prush.

```rust
// Define Push program
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

For existing types the instruction set can be extended by calling the ``add`` function.


```rust
pub fn my_instruction(_push_state: &mut PushState, _instruction_set: &InstructionCache) {
    // Does nothing
}

...

let mut instruction_set = InstructionSet::new();
instruction_set.add(String::from("MyInstruction"), Instruction::new(my_instruction));

```





