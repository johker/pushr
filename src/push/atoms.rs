// Atoms
pub enum Atom {
    CodeBlock,
    Closer,
    InstructionMeta(InstructionMeta),
    Literal(Literal),
    Input,
}

pub struct InstructionMeta {
    name: String,
    // Number of code_blocks that are expected to follow the instruction
    code_blocks: i32,
}

pub struct Literal {
    pub pushType: PushType,
}

pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}
