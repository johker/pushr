// Atoms
#[derive(Clone)]
pub enum Atom {
    CodeBlock,
    Closer,
    InstructionMeta { name: String, code_blocks: u32 },
    Literal { push_type: PushType },
    Input,
}

#[derive(Clone)]
pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}
