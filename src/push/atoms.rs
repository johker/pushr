// Atoms
#[derive(Clone, Copy)]
pub enum Atom<'a> {
    CodeBlock,
    Closer,
    InstructionMeta { name: &'a str, code_blocks: u32 },
    Literal { push_type: PushType },
    Input,
}

#[derive(Clone, Copy)]
pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}

struct InstructionMeta {
    name: String,
    code_blocks: u32,
}
