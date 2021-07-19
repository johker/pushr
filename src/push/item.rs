use std::fmt;

use crate::push::stack::PushStack;

// Items
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Item<'a> {
    List { items: PushStack<Item<'a>> },
    Closer,
    InstructionMeta { name: &'a str },
    Literal { push_type: PushType },
    Identifier { name: &'a str },
}

#[derive(Clone, PartialEq, Debug)]
pub enum PushType {
    PushBoolType { val: bool },
    PushIntType { val: i32 },
    PushFloatType { val: f32 },
}

#[allow(dead_code)]
impl<'a> Item<'a> {
    pub fn int(arg: i32) -> Item<'a> {
        Item::Literal {
            push_type: PushType::PushIntType { val: arg },
        }
    }
    pub fn float(arg: f32) -> Item<'a> {
        Item::Literal {
            push_type: PushType::PushFloatType { val: arg },
        }
    }
    pub fn bool(arg: bool) -> Item<'a> {
        Item::Literal {
            push_type: PushType::PushBoolType { val: arg },
        }
    }
    pub fn noop() -> Item<'a> {
        Item::InstructionMeta { name: "NOOP" }
    }
    pub fn empty_list() -> Item<'a> {
        Item::List {
            items: PushStack::new(),
        }
    }
    pub fn list(arg: Vec<Item<'a>>) -> Item<'a> {
        Item::List {
            items: PushStack::from_vec(arg),
        }
    }
    pub fn id(arg: &'a str) -> Item<'a> {
        Item::Identifier { name: arg }
    }

    /// Returns a nested element of a list using depth first traversal.
    pub fn traverse(item: &Item<'a>, mut depth: usize) -> Result<Item<'a>, usize> {
        if depth == 0 {
            Ok(item.clone())
        } else {
            match item {
                Item::List { items } => {
                    for i in 0..items.size() {
                        depth -= 1;
                        let next = Item::traverse(&items.observe(i).unwrap(), depth);
                        match next {
                            Ok(next) => return Ok(next),
                            Err(new_depth) => depth = new_depth,
                        }
                    }
                }
                _ => (),
            }
            Err(depth)
        }
    }

    /// Replaces a nested element of a list using depth first traversal.
    pub fn insert(item: &mut Item<'a>, new_el: &Item<'a>, mut depth: usize) -> Result<bool, usize> {
        if depth == 0 {
            Ok(true)
        } else {
            match &mut *item {
                Item::List { items } => {
                    let replace_idx = depth - 1;
                    for i in 0..items.size() {
                        depth -= 1;
                        let next = Item::insert(items.get_mut(i).unwrap(), new_el, depth);
                        match next {
                            Ok(replace_here) => {
                                if replace_here {
                                    let _ = items.replace(replace_idx, new_el.clone());
                                }
                                return Ok(false);
                            }
                            Err(new_depth) => depth = new_depth,
                        }
                    }
                }
                _ => (),
            }
            Err(depth)
        }
    }
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Self) -> bool {
        match &*self {
            Item::List { items: _ } => match &*other {
                Item::List { items: _ } => return true,
                _ => return false,
            },
            Item::Closer => match &*other {
                Item::Closer => return true,
                _ => return false,
            },
            Item::InstructionMeta { name: _ } => match &*other {
                Item::InstructionMeta { name: _ } => return true,
                _ => return false,
            },
            Item::Literal { push_type: _ } => match &*other {
                Item::Literal { push_type: _ } => return true,
                _ => return false,
            },
            Item::Identifier { name: _ } => match &*other {
                Item::Identifier { name: _ } => return true,
                _ => return false,
            },
        }
    }
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Item::List { items } => write!(f, "List: {}", items.to_string()),
            Item::Closer => write!(f, "Closer"),
            Item::InstructionMeta { name } => {
                let at = "InstructionMeta".to_string();
                write!(f, "{}({})", at, name)
            }
            Item::Literal { push_type } => {
                let at = "Literal".to_string();
                let info;
                match push_type {
                    PushType::PushBoolType { val } => info = val.to_string(),
                    PushType::PushIntType { val } => info = val.to_string(),
                    PushType::PushFloatType { val } => info = val.to_string(),
                }
                write!(f, "{}({})", at, info)
            }
            Item::Identifier { name } => {
                let at = "Identifier".to_string();
                write!(f, "{}({})", at, name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shallow_equality_returns_true_comparing_items_with_different_content() {
        let literal_a = Item::int(0);
        let literal_b = Item::int(2);
        let closer_a = Item::Closer;
        let closer_b = Item::Closer;
        let list_a = Item::list(vec![Item::Closer]);
        let list_b = Item::list(vec![Item::int(0)]);
        let inst_a = Item::noop();
        let inst_b = Item::InstructionMeta {
            name: "BOOLEAN.AND",
        };
        assert_eq!(list_a, list_b);
        assert_eq!(inst_a, inst_b);
        assert_eq!(literal_a, literal_b);
        assert_eq!(closer_a, closer_b);
        assert_ne!(list_a, literal_b);
        assert_ne!(closer_a, literal_b);
    }

    #[test]
    fn print_list_shows_sublements() {
        let list = Item::list(vec![Item::int(0), Item::int(1)]);
        assert_eq!(list.to_string(), "List: 1:Literal(1); 2:Literal(0);");
    }

    #[test]
    fn traverse_returns_right_element_sublist() {
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        assert_eq!(
            Item::traverse(&test_item, 4).unwrap().to_string(),
            "Literal(3)"
        );
    }

    #[test]
    fn insert_replaces_element_at_given_index() {
        let mut test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let item_to_insert = Item::int(99);
        assert_eq!(Item::insert(&mut test_item, &item_to_insert, 4), Ok(false));
        assert_eq!(
            test_item.to_string(),
            "List: 1:Literal(1); 2:Literal(2); 3:List: 1:Literal(99);; 4:Literal(4);"
        );
    }

    #[test]
    fn insert_returns_error_for_too_code_containers() {
        let mut test_item = Item::int(1);
        let item_to_insert = Item::int(99);
        assert_eq!(Item::insert(&mut test_item, &item_to_insert, 4), Err(4));
    }
}
