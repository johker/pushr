// Atoms
#[derive(Clone)]
pub enum Atom {
    CodeBlock,
    Closer,
    InstructionMeta { name: String, code_blocks: i32 },
    Literal(Literal),
    Input,
}

#[derive(Clone)]
pub struct Literal {
    pub pushType: PushType,
}

#[derive(Clone)]
pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}
