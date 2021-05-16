use std::fmt;

// Atoms
#[derive(Clone, Debug)]
pub enum Atom<'a> {
    CodeBlock { atoms: Vec<Atom<'a>> },
    Closer,
    InstructionMeta { name: &'a str, code_blocks: u32 },
    Literal { push_type: PushType },
    Input,
}

#[derive(Clone, PartialEq, Debug)]
pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}

impl<'a> PartialEq for Atom<'a> {
    fn eq(&self, other: &Self) -> bool {
        match &*self {
            Atom::CodeBlock { atoms: _ } => match &*other {
                Atom::CodeBlock { atoms: _ } => return true,
                _ => return false,
            },
            Atom::Closer => match &*other {
                Atom::Closer => return true,
                _ => return false,
            },
            Atom::InstructionMeta {
                name,
                code_blocks: _,
            } => match &*other {
                Atom::InstructionMeta {
                    name: oname,
                    code_blocks: _,
                } => return name == oname,
                _ => return false,
            },
            Atom::Literal { push_type } => match &*other {
                Atom::Literal {
                    push_type: opush_type,
                } => return push_type == opush_type,
                _ => return false,
            },
            Atom::Input => match &*other {
                Atom::Input => return true,
                _ => return false,
            },
        }
    }
}

impl<'a> fmt::Display for Atom<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Atom::CodeBlock { atoms } => {
                let mut atom_info = String::new();
                for (i, a) in atoms.iter().rev().enumerate() {
                    atom_info.push_str(&format!("{}:{}; ", (i + 1), a));
                }
                write!(f, "CodeBlock; {}", atom_info.trim())
            }
            Atom::Closer => write!(f, "Closer"),
            Atom::InstructionMeta {
                name,
                code_blocks: _,
            } => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shallow_equality() {
        let literal_a = Atom::Literal {
            push_type: PushType::PushIntType { val: 0 },
        };
        let literal_b = Atom::Literal {
            push_type: PushType::PushIntType { val: 2 },
        };
        let closer_a = Atom::Closer;
        let closer_b = Atom::Closer;
        let code_block_a = Atom::CodeBlock {
            atoms: vec![Atom::Closer],
        };
        let code_block_b = Atom::CodeBlock {
            atoms: vec![Atom::Literal {
                push_type: PushType::PushIntType { val: 0 },
            }],
        };
        assert_eq!(code_block_a, code_block_b);
        assert_ne!(literal_a, literal_b);
        assert_eq!(closer_a, closer_b);
        assert_ne!(code_block_b, literal_b);
        assert_ne!(closer_a, literal_b);
    }

    #[test]
    fn test_display_code_block() {
        let code_block = Atom::CodeBlock {
            atoms: vec![Atom::Literal {
                push_type: PushType::PushIntType { val: 0 },
            }],
        };
        assert_eq!(code_block.to_string(), "CodeBlock; 1:Literal(0);");
    }
}
