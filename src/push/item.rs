use std::fmt;

use crate::push::index::Index;
use crate::push::stack::PushStack;
use crate::push::vector::{BoolVector, FloatVector, IntVector};

// Items
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Item {
    List { items: PushStack<Item> },
    InstructionMeta { name: String },
    Literal { push_type: PushType },
    Identifier { name: String },
}

#[derive(Clone, PartialEq, Debug)]
pub enum PushType {
    Bool { val: bool },
    Int { val: i32 },
    Index { val: Index },
    Float { val: f32 },
    BoolVector { val: BoolVector },
    IntVector { val: IntVector },
    FloatVector { val: FloatVector },
}

#[allow(dead_code)]
impl<'a> Item {
    pub fn int(arg: i32) -> Item {
        Item::Literal {
            push_type: PushType::Int { val: arg },
        }
    }
    pub fn index(arg: Index) -> Item {
        Item::Literal {
            push_type: PushType::Index { val: arg },
        }
    }
    pub fn float(arg: f32) -> Item {
        Item::Literal {
            push_type: PushType::Float { val: arg },
        }
    }
    pub fn bool(arg: bool) -> Item {
        Item::Literal {
            push_type: PushType::Bool { val: arg },
        }
    }

    pub fn boolvec(arg: BoolVector) -> Item {
        Item::Literal {
            push_type: PushType::BoolVector { val: arg },
        }
    }

    pub fn floatvec(arg: FloatVector) -> Item {
        Item::Literal {
            push_type: PushType::FloatVector { val: arg },
        }
    }

    pub fn intvec(arg: IntVector) -> Item {
        Item::Literal {
            push_type: PushType::IntVector { val: arg },
        }
    }

    pub fn instruction(arg: String) -> Item {
        Item::InstructionMeta { name: arg }
    }

    pub fn name(arg: String) -> Item {
        Item::Identifier { name: arg }
    }

    pub fn noop() -> Item {
        Item::InstructionMeta {
            name: "NOOP".to_string(),
        }
    }
    pub fn empty_list() -> Item {
        Item::List {
            items: PushStack::new(),
        }
    }
    pub fn list(arg: Vec<Item>) -> Item {
        Item::List {
            items: PushStack::from_vec(arg),
        }
    }
    pub fn id(arg: String) -> Item {
        Item::Identifier { name: arg }
    }

    /// Returns the number of elements where each parenthesized expression and each
    /// literal/instruction is considered a point. It proceeds in depth first order.
    pub fn size(item: &Item) -> usize {
        let mut size = 0;
        match item {
            Item::List { items } => {
                size += 1;
                for i in 0..items.size() {
                    size += Item::size(&items.get(i).unwrap());
                }
            }
            _ => size += 1,
        }
        return size;
    }

    /// Returns the number of elements the items cotains up to a depth of 1.
    pub fn shallow_size(item: &Item) -> usize {
        let mut size = 0;
        match item {
            Item::List { items } => {
                size += items.size() + 1;
            }
            _ => size += 1,
        }
        return size;
    }

    /// Returns a nested element of a list using depth first traversal.
    pub fn traverse(item: &Item, mut depth: usize) -> Result<Item, usize> {
        if depth == 0 {
            Ok(item.clone())
        } else {
            match item {
                Item::List { items } => {
                    for i in 0..items.size() {
                        depth -= 1;
                        let next = Item::traverse(&items.copy(i).unwrap(), depth);
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
    pub fn insert(item: &mut Item, new_el: &Item, mut depth: usize) -> Result<bool, usize> {
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

    /// Substitute all occurrences of 'pattern' with 'substitute' in 'item' using depth first
    /// traversal.
    pub fn substitute(item: &mut Item, pattern: &Item, substitute: &Item) -> bool {
        if Item::equals(item, pattern) {
            return true;
        } else {
            match &mut *item {
                Item::List { items } => {
                    for i in 0..items.size() {
                        if Item::substitute(items.get_mut(i).unwrap(), pattern, substitute) {
                            let _ = items.replace(i, substitute.clone());
                        }
                    }
                }
                _ => (),
            }
            return false;
        }
    }

    /// Returns the position of pattern within item or Err if pattern is not
    /// part of item
    pub fn contains(item: &Item, pattern: &Item, mut depth: usize) -> Result<usize, ()> {
        if Item::equals(item, pattern) {
            Ok(depth)
        } else {
            match item {
                Item::List { items } => {
                    for i in 0..items.size() {
                        depth += 1;
                        let next = Item::contains(items.get(i).unwrap(), pattern, depth);
                        match next {
                            Ok(pattern_idx) => return Ok(pattern_idx),
                            Err(()) => (),
                        }
                    }
                }
                _ => (),
            }
            Err(())
        }
    }
    /// Returns the container of pattern within item, i.e. its smallest sublist that contains but
    /// is not equal to pattern. It returns Err if pattern is not part of item
    pub fn container(item: &Item, pattern: &Item) -> Result<Item, bool> {
        if Item::equals(item, pattern) {
            Err(true)
        } else {
            match item {
                Item::List { items } => {
                    for i in 0..items.size() {
                        let next = Item::container(items.get(i).unwrap(), pattern);
                        match next {
                            Ok(container) => return Ok(container),
                            Err(is_container) => {
                                if is_container {
                                    return Ok(item.clone());
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
            Err(false)
        }
    }

    /// Executes a deep comparison between two item. Returns true if
    /// the items and all their elements are equal.
    pub fn equals(item: &Item, pattern: &Item) -> bool {
        match item {
            Item::List { items } => match &*pattern {
                Item::List { items: pitems } => {
                    if items.size() != pitems.size() {
                        return false;
                    }
                    for i in 0..items.size() {
                        if !Item::equals(items.get(i).unwrap(), pitems.get(i).unwrap()) {
                            return false;
                        }
                    }
                    true
                }
                _ => false,
            },
            Item::InstructionMeta { name } => match pattern {
                Item::InstructionMeta { name: pname } => name == pname,
                _ => false,
            },
            Item::Literal { push_type } => match pattern {
                Item::Literal { push_type: ptype } => push_type.equals(ptype),
                _ => false,
            },
            Item::Identifier { name } => match pattern {
                Item::Identifier { name: pname } => name == pname,
                _ => false,
            },
        }
    }
}

impl<'a> PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match &*self {
            Item::List { items: _ } => match &*other {
                Item::List { items: _ } => return true,
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

impl<'a> fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Item::List { items } => write!(f, "List: {}", items.to_string()),
            Item::InstructionMeta { name } => {
                let at = "InstructionMeta".to_string();
                write!(f, "{}({})", at, name)
            }
            Item::Literal { push_type } => {
                let at = "Literal".to_string();
                let info;
                match push_type {
                    PushType::Bool { val } => info = val.to_string(),
                    PushType::Int { val } => info = val.to_string(),
                    PushType::Index { val } => info = val.to_string(),
                    PushType::Float { val } => info = val.to_string() + "f",
                    PushType::BoolVector { val } => info = val.to_string(),
                    PushType::FloatVector { val } => info = val.to_string(),
                    PushType::IntVector { val } => info = val.to_string(),
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

impl PushType {
    /// Returns true if type and value are equal
    pub fn equals(&self, other: &PushType) -> bool {
        match &*self {
            PushType::Bool { val } => match &*other {
                PushType::Bool { val: other_val } => return val == other_val,
                _ => false,
            },
            PushType::Int { val } => match &*other {
                PushType::Int { val: other_val } => return val == other_val,
                _ => false,
            },
            PushType::Index { val } => match &*other {
                PushType::Index { val: other_val } => return val == other_val,
                _ => false,
            },
            PushType::Float { val } => match &*other {
                PushType::Float { val: other_val } => return val == other_val,
                _ => false,
            },
            PushType::BoolVector { val } => match &*other {
                PushType::BoolVector { val: other_val } => return val == other_val,
                _ => false,
            },
            PushType::FloatVector { val } => match &*other {
                PushType::FloatVector { val: other_val } => return val == other_val,
                _ => false,
            },
            PushType::IntVector { val } => match &*other {
                PushType::IntVector { val: other_val } => return val == other_val,
                _ => false,
            },
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
        let list_a = Item::list(vec![Item::float(3.4)]);
        let list_b = Item::list(vec![Item::int(0)]);
        let inst_a = Item::noop();
        let inst_b = Item::InstructionMeta {
            name: "BOOLEAN.AND".to_string(),
        };
        assert_eq!(list_a, list_b);
        assert_eq!(inst_a, inst_b);
        assert_eq!(literal_a, literal_b);
        assert_ne!(list_a, literal_b);
        assert_ne!(inst_b, literal_b);
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
    fn insert_returns_error_for_index_out_of_bounds() {
        let mut test_item = Item::int(1);
        let item_to_insert = Item::int(99);
        assert_eq!(Item::insert(&mut test_item, &item_to_insert, 4), Err(4));
    }

    #[test]
    fn size_includes_nested_lists_in_count() {
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        assert_eq!(Item::size(&test_item), 6);
    }

    #[test]
    fn shallow_size_only_considers_depth_1() {
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        assert_eq!(Item::shallow_size(&test_item), 5);
    }

    #[test]
    fn equals_returns_true_for_deep_equality() {
        let i1 = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let i2 = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        assert!(Item::equals(&i1, &i2));
    }

    #[test]
    fn equals_detects_non_matching_sub_lists() {
        let i1 = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let i2 = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(2)]),
            Item::int(2),
            Item::int(1),
        ]);
        assert!(!Item::equals(&i1, &i2));
    }

    #[test]
    fn contains_finds_index_of_sublist() {
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let pattern = Item::list(vec![Item::int(3)]);
        assert_eq!(Item::contains(&test_item, &pattern, 0), Ok(3));
        assert_eq!(Item::contains(&test_item, &Item::int(1), 0), Ok(1));
    }

    #[test]
    fn container_finds_sublist() {
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let pattern = Item::int(3);
        assert!(Item::equals(
            &Item::container(&test_item, &pattern).unwrap(),
            &Item::list(vec![Item::int(3)])
        ));
    }

    #[test]
    fn contains_returns_error_if_sublist_not_contained() {
        let test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let pattern = Item::list(vec![Item::int(5)]);
        assert_eq!(Item::contains(&test_item, &pattern, 0), Err(()));
    }

    #[test]
    fn substitute_with_literal_pattern() {
        let mut test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let pattern = Item::int(3);
        let substitute = Item::int(9);
        Item::substitute(&mut test_item, &pattern, &substitute);
        assert_eq!(
            test_item.to_string(),
            "List: 1:Literal(1); 2:Literal(2); 3:List: 1:Literal(9);; 4:Literal(4);"
        );
    }
    #[test]
    fn substitute_with_list_pattern() {
        let mut test_item = Item::list(vec![
            Item::int(4),
            Item::list(vec![Item::int(3)]),
            Item::int(2),
            Item::int(1),
        ]);
        let pattern = Item::list(vec![Item::int(3)]);
        let substitute = Item::int(9);
        Item::substitute(&mut test_item, &pattern, &substitute);
        assert_eq!(
            test_item.to_string(),
            "List: 1:Literal(1); 2:Literal(2); 3:Literal(9); 4:Literal(4);"
        );
    }
}
