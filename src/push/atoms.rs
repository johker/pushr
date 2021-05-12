use std::fmt;

// Atoms
#[derive(Clone)]
pub enum Atom<'a> {
    CodeBlock { atoms: Vec<Atom<'a>> },
    Closer,
    InstructionMeta { name: &'a str, code_blocks: u32 },
    Literal { push_type: PushType },
    Input,
}

#[derive(Clone)]
pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}

impl<'a> fmt::Display for Atom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Atom::CodeBlock { atoms } => write!(f, "CodeBlock"),
            Atom::Closer => write!(f, "Closer"),
            Atom::InstructionMeta { name, code_blocks } => {
                let at = "InstructionMeta".to_string();
                write!(f, "{}({})", at, name)
            }
            Atom::Literal { push_type } => {
                let at = "Literal".to_string();
                let info;
                match push_type {
                    PushType::PushBoolType { val } => info = val.to_string(),
                    PushType::PushIntType { val } => info = val.to_string(),
                    PushType::PushFloatType { val } => info = val.to_string(),
                }
                write!(f, "{}({})", at, info)
            }
            Atom::Input => write!(f, "Input"),
        }
    }
}

struct InstructionMeta {
    name: String,
    code_blocks: u32,
}
