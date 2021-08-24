pub struct PushConfiguration {
    // The maximum FLOAT that will be produced as an ephemeral random FLOAT constant or from a call to FLOAT.RAND.
    pub max_random_float: f32,
    // The minimum FLOAT that will be produced as an ephemeral random FLOAT constant or from a
    // call to FLOAT.RAND.
    pub min_random_float: f32,
    // The maximum INTEGER that will be produced as an ephemeral random INTEGER constant or from a
    // call to INTEGER.RAND.
    pub max_random_integer: i32,
    // The minimum INTEGER that will be produced as an ephemeral random INTEGER constant or from a
    // call to INTEGER.RAND.
    pub min_random_integer: i32,
    // The maximum number of points that will be executed in a single top-level call to the
    // interpreter.
    pub eval_push_limit: i32,
    // The maximum time in milliseconds for the execution of a single top-level call to the interpreter
    pub eval_time_limit: u64,
    // Max number of elements that can be added to a PushState at any given
    // step of program execution. If exceeded, program terminates.
    pub growth_cap: usize,
    // The probability that the selection of the ephemeral
    // random NAME constant for inclusion in randomly generated code will produce a new name
    // (rather than a name that was previously generated).
    pub new_erc_name_probability: f32,
    // The maximum number of points in an expression produced by the CODE.RAND instruction.
    pub max_points_in_random_expressions: i32,
    // The maximum number of points that can occur in any program on the CODE stack. Instructions
    // that would violate this limit act as NOOPs (they do nothing).
    pub max_points_in_program: i32,
}

impl PushConfiguration {
    pub fn new() -> Self {
        Self {
            max_random_float: 1.0,
            min_random_float: -1.0,
            max_random_integer: 10,
            min_random_integer: -10,
            eval_push_limit: 1000,
            eval_time_limit: 5000,
            growth_cap: 500,
            new_erc_name_probability: 0.001,
            max_points_in_random_expressions: 25,
            max_points_in_program: 100,
        }
    }
}
