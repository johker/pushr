use crate::push::instructions::{Instruction, InstructionCache};
use crate::push::item::Item;
use crate::push::item::PushType;
use crate::push::state::*;
use crate::push::topology::Topology;
use crate::push::vector::{BoolVector, FloatVector, IntVector};
use std::collections::HashMap;

/// Integer numbers (that is, numbers without decimal points).
pub fn load_list_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("LIST.ADD"), Instruction::new(list_add));
    map.insert(String::from("LIST.REMOVE"), Instruction::new(list_remove));
    map.insert(String::from("LIST.GET"), Instruction::new(list_get));
    map.insert(String::from("LIST.SET"), Instruction::new(list_set));
    map.insert(String::from("LIST.BVAL"), Instruction::new(list_bval));
    map.insert(String::from("LIST.IVAL"), Instruction::new(list_ival));
    map.insert(String::from("LIST.FVAL"), Instruction::new(list_fval));
    map.insert(
        String::from("LIST.NEIGHBOR*IDS"),
        Instruction::new(list_neighbor_ids),
    );
    map.insert(
        String::from("LIST.NEIGHBOR*BVALS"),
        Instruction::new(list_neighbor_bvals),
    );
    map.insert(
        String::from("LIST.NEIGHBOR*IVALS"),
        Instruction::new(list_neighbor_ivals),
    );
    map.insert(
        String::from("LIST.NEIGHBOR*FVALS"),
        Instruction::new(list_neighbor_fvals),
    );
}

/// Returns the nth integer that is contained in the item.
/// If no such value exists it returns 0
pub fn bval(item: &Item, n: &usize) -> bool {
    let default = false;
    match Item::find(item, &Item::bool(false), &mut 0, n) {
        Ok(bval) => match bval {
            Item::Literal { push_type } => match push_type {
                PushType::Bool { val } => return val,
                _ => (),
            },
            _ => (),
        },
        Err(_cnt) => (),
    };
    return default;
}

/// Returns the first integer that is contained in the item.
/// If no such value exists it returns 0
pub fn ival(item: &Item, n: &usize) -> i32 {
    let default = 0;
    match Item::find(item, &Item::int(0), &mut 0, n) {
        Ok(ival) => match ival {
            Item::Literal { push_type } => match push_type {
                PushType::Int { val } => return val,
                _ => (),
            },
            _ => (),
        },
        Err(_cnt) => (),
    };
    return default;
}

/// Returns the first float that is contained in the item.
/// If no such value exists it returns 0
pub fn fval(item: &Item, n: &usize) -> f32 {
    let default = 0.0;
    match Item::find(item, &Item::float(0.0), &mut 0, n) {
        Ok(fval) => match fval {
            Item::Literal { push_type } => match push_type {
                PushType::Float { val } => return val,
                _ => (),
            },
            _ => (),
        },
        Err(_cnt) => (),
    };
    return default;
}

/// Generates a vector of items as specified by the top INTVECTOR.
/// Each entry is matched against the stack ids. If there is a match the item
/// of the stack is popped and added to the new list item. As last entry
/// it adds a auto-generated ID.
pub fn load_items(push_state: &mut PushState) -> Option<Vec<Item>> {
    if let Some(stack_ids) = push_state.int_vector_stack.pop() {
        let mut items = vec![];
        for &sid in &stack_ids.values {
            match sid {
                BOOL_STACK_ID => {
                    if let Some(bi) = push_state.bool_stack.pop() {
                        items.push(Item::bool(bi));
                    }
                }
                BOOL_VECTOR_STACK_ID => {
                    if let Some(bvi) = push_state.bool_vector_stack.pop() {
                        items.push(Item::boolvec(bvi));
                    }
                }
                CODE_STACK_ID => {
                    if let Some(ci) = push_state.code_stack.pop() {
                        items.push(ci);
                    }
                }
                EXEC_STACK_ID => {
                    if let Some(ei) = push_state.exec_stack.pop() {
                        items.push(ei);
                    }
                }
                FLOAT_STACK_ID => {
                    if let Some(fi) = push_state.float_stack.pop() {
                        items.push(Item::float(fi));
                    }
                }
                FLOAT_VECTOR_STACK_ID => {
                    if let Some(fvi) = push_state.float_vector_stack.pop() {
                        items.push(Item::floatvec(fvi));
                    }
                }
                INT_STACK_ID => {
                    if let Some(ii) = push_state.int_stack.pop() {
                        items.push(Item::int(ii));
                    }
                }
                INT_VECTOR_STACK_ID => {
                    if let Some(ivi) = push_state.int_vector_stack.pop() {
                        items.push(Item::intvec(ivi));
                    }
                }
                NAME_STACK_ID => {
                    if let Some(ni) = push_state.name_stack.pop() {
                        items.push(Item::name(ni));
                    }
                }
                _ => (),
            }
        }
        return Some(items);
    }
    return None;
}

/// Generates a new list using the items specified on top of the INTVECTOR stack.
pub fn new_list(push_state: &mut PushState) -> Option<Vec<Item>> {
    if let Some(items) = load_items(push_state) {
        // items.reverse();
        return Some(items);
    }
    return None;
}

/// LIST.ADD: Pushes a list item to the code stack with the content
/// specified by the top item of the INTVECTOR. Each entry of the INTVECTOR
/// represents the stack id of an item to be contained.
pub fn list_add(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    if let Some(items) = new_list(push_state) {
        let list_item = Item::list(items);
        push_state.code_stack.push(list_item);
    }
}

/// LIST.REMOVE: Removes the list item at index i.
/// The index i is taken from the top of the INTEGER stack and min-max corrected.
pub fn list_remove(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index), 0) as usize;
        push_state.code_stack.remove(list_index);
    }
}

/// LIST.GET: Pushes a copy of the items at the given stack position to the execution stack.
/// The first element of the list (the id) is removed.
/// The index i is taken from the top of the INTEGER stack and min-max corrected.
pub fn list_get(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index), 0) as usize;
        if let Some(list) = push_state.code_stack.copy(list_index) {
            match list {
                Item::List { items } => {
                    // items.reverse();
                    push_state.exec_stack.push(Item::List { items: items });
                }
                _ => (),
            }
        }
    }
}

/// LIST.BVAL: Pushes the nth BOOLEAN contained in the list item at stack position i.
/// The index i is taken from the top of the INTEGER stack and min-max corrected.
pub fn list_bval(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop_vec(2) {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index[0]), 0) as usize;
        if let Some(list_item) = push_state.code_stack.get(list_index) {
            push_state
                .bool_stack
                .push(bval(list_item, &(index[1] as usize)));
        }
    }
}

/// LIST.IVAL: Pushes the nth INTEGER contained in the list item at stack position i.
/// The index i is taken from the top of the INTEGER stack and min-max corrected.
pub fn list_ival(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop_vec(2) {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index[0]), 0) as usize;
        if let Some(list_item) = push_state.code_stack.get(list_index) {
            push_state
                .int_stack
                .push(ival(list_item, &(index[1] as usize)));
        }
    }
}

/// LIST.FVAL: Pushes the nth INTEGER contained in the list item at stack position i.
/// The index i is taken from the top of the INTEGER stack and min-max corrected.
pub fn list_fval(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop_vec(2) {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index[0]), 0) as usize;
        if let Some(list_item) = push_state.code_stack.get(list_index) {
            push_state
                .float_stack
                .push(fval(list_item, &(index[1] as usize)));
        }
    }
}

/// LIST.SET: Replaces the items bound to the specified index. The list index is taken
/// from the top of the INTEGER stack and min-max corrected. The content is taken from
/// the stacks that are identified by the top INTVECTOR element. For example
/// [ INTEGER.ID INTEGER.ID BOOLEAN.ID ] = [ 9 9 1 ] replaces with a list containing
/// two integer and a boolean item.
pub fn list_set(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index), 0) as usize;
        if let Some(items) = load_items(push_state) {
            // items.reverse();
            let list_item = Item::list(items);
            let _res = push_state.code_stack.replace(list_index, list_item);
        }
    }
}

/// LIST.NEIGHBORS*ID: Calculates the neighborhood for a given index element and length. It
/// pushes the indices that are contained in this neighborhood to the INTVECTOR stack.
/// The size, the number of dimensions and index (vector topology) are taken from the INTEGER
/// stack in that order. The radius is taken from the float stack. Distances are calculated using the
/// Eucledian metric. All values are corrected by max-min. If the size of the top element is not a power
/// of the dimensions the smallest hypercube that includes the indices is used to represent the
/// topology, e.g. two dimensions and size = 38 is represented by[7,7]. Neighbor indices that
/// do no exist (e.g. 40) are ignored.
pub fn list_neighbor_ids(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(topology) = push_state.int_stack.pop_vec(3) {
        let size = i32::max(topology[2], 0);
        let index = i32::max(i32::min(size - 1, topology[1]), 0) as usize;
        let dimensions = i32::max(i32::min(size, topology[0]), 0) as usize;
        if let Some(fval) = push_state.float_stack.pop() {
            let radius = f32::max(fval, 0.0);
            if let Some(neighbors) =
                Topology::find_neighbors(&(size as usize), &dimensions, &index, &radius)
            {
                let mut result = vec![];
                for n in neighbors.values.iter() {
                    result.push(*n);
                }
                push_state.int_vector_stack.push(IntVector::new(result));
            }
        }
    }
}

/// LIST.NEIGHBOR*BVALS: Pushes the sorting value of the neighborhood for a given index to the
/// BOOLVECTOR stack. The neighborhood is calculated as in LIST.NEIGHBOR*IDS.
pub fn list_neighbor_bvals(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(topology) = push_state.int_stack.pop_vec(4) {
        let position = topology[3] as usize;
        let size = i32::max(topology[2], 0);
        let index = i32::max(i32::min(size - 1, topology[1]), 0) as usize;
        let dimensions = i32::max(i32::min(size, topology[0]), 0) as usize;
        if let Some(fval) = push_state.float_stack.pop() {
            let radius = f32::max(fval, 0.0);
            if let Some(neighbors) =
                Topology::find_neighbors(&(size as usize), &dimensions, &index, &radius)
            {
                let mut result = vec![];
                for n in neighbors.values.iter() {
                    if let Some(item) = push_state.code_stack.get(*n as usize) {
                        result.push(bval(item, &position));
                    }
                }
                push_state.bool_vector_stack.push(BoolVector::new(result));
            }
        }
    }
}

/// LIST.NEIGHBOR*IVALS: Pushes the sorting value of the neighborhood for a given index to the
/// INTVECTOR stack. The neighborhood is calculated as in LIST.NEIGHBOR*IDS.
pub fn list_neighbor_ivals(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(topology) = push_state.int_stack.pop_vec(4) {
        let position = topology[3] as usize;
        let size = i32::max(topology[2], 0);
        let index = i32::max(i32::min(size - 1, topology[1]), 0) as usize;
        let dimensions = i32::max(i32::min(size, topology[0]), 0) as usize;
        if let Some(fval) = push_state.float_stack.pop() {
            let radius = f32::max(fval, 0.0);
            if let Some(neighbors) =
                Topology::find_neighbors(&(size as usize), &dimensions, &index, &radius)
            {
                let mut result = vec![];
                for n in neighbors.values.iter() {
                    if let Some(item) = push_state.code_stack.get(*n as usize) {
                        result.push(ival(item, &position));
                    }
                }
                push_state.int_vector_stack.push(IntVector::new(result));
            }
        }
    }
}

/// LIST.NEIGHBOR*FVALS: Pushes the sorting value of the neighborhood for a given index to the
/// FLOATVECTOR stack. The neighborhood is calculated as in LIST.NEIGHBOR*IDS.
pub fn list_neighbor_fvals(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(topology) = push_state.int_stack.pop_vec(4) {
        let position = topology[3] as usize;
        let size = i32::max(topology[2], 0);
        let index = i32::max(i32::min(size - 1, topology[1]), 0) as usize;
        let dimensions = i32::max(i32::min(size, topology[0]), 0) as usize;
        if let Some(rval) = push_state.float_stack.pop() {
            let radius = f32::max(rval, 0.0);
            if let Some(neighbors) =
                Topology::find_neighbors(&(size as usize), &dimensions, &index, &radius)
            {
                let mut result = vec![];
                for n in neighbors.values.iter() {
                    if let Some(item) = push_state.code_stack.get(*n as usize) {
                        result.push(fval(item, &position));
                    }
                }
                push_state.float_vector_stack.push(FloatVector::new(result));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::instructions::InstructionSet;
    use crate::push::interpreter::{PushInterpreter, PushInterpreterState};
    use crate::push::vector::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    /// Creates a test list entry with the given
    /// value to sort.
    pub fn litem(i: i32) -> Item {
        Item::list(vec![Item::int(i)])
    }

    #[test]
    fn list_add_from_different_stacks() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.int_stack.push(1);
        test_state.float_stack.push(1.0);
        test_state
            .float_vector_stack
            .push(FloatVector::new(vec![3.0]));
        test_state
            .bool_vector_stack
            .push(BoolVector::from_int_array(vec![1]));
        test_state.int_vector_stack.push(IntVector::new(vec![22]));
        test_state.int_vector_stack.push(IntVector::new(vec![
            BOOL_STACK_ID,
            FLOAT_STACK_ID,
            FLOAT_VECTOR_STACK_ID,
            BOOL_VECTOR_STACK_ID,
            INT_VECTOR_STACK_ID,
            INT_STACK_ID,
        ]));
        list_add(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "1:List: 1:Literal(1); 2:Literal([22]); 3:Literal([1]); 4:Literal([3]); 5:Literal(1f); 6:Literal(true);;");
    }

    #[test]
    fn list_remove_code_items() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::list(vec![
            Item::bool(true),
            Item::int(2),
            Item::int(3),
            Item::float(2.3),
        ]));
        test_state.code_stack.push(Item::int(2));
        test_state.int_stack.push(1);
        list_remove(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:Literal(2); 2:Literal(1);"
        );
    }

    #[test]
    fn list_get_pushes_code_items() {
        let mut test_state = PushState::new();
        test_state.code_stack.push(Item::int(1));
        test_state.code_stack.push(Item::list(vec![
            Item::bool(true),
            Item::int(2),
            Item::int(3),
            Item::float(2.3),
        ]));
        test_state.code_stack.push(Item::int(2));
        test_state.int_stack.push(1);
        list_get(&mut test_state, &icache());
        assert_eq!(
            test_state.exec_stack.to_string(),
            "1:List: 1:Literal(2.3f); 2:Literal(3); 3:Literal(2); 4:Literal(true);;",
            "Order of elements should be reversed"
        );
    }

    #[test]
    fn list_bval_returns_first_int_element() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Item::list(vec![Item::bool(true), Item::int(2)]));
        test_state.int_stack.push(0); // Stack Position
        test_state.int_stack.push(0); // Item Position
        list_bval(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn list_ival_returns_first_int_element() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Item::list(vec![Item::int(1), Item::int(2)]));
        test_state.int_stack.push(0); // Stack Position
        test_state.int_stack.push(0); // Item Position
        list_ival(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 2);
    }

    #[test]
    fn list_fval_returns_first_float_element() {
        let mut test_state = PushState::new();
        test_state
            .code_stack
            .push(Item::list(vec![Item::float(1.0), Item::int(2)]));
        test_state.int_stack.push(0); // Stack Position
        test_state.int_stack.push(0); // Item Position
        list_fval(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.pop().unwrap(), 1.0);
    }

    #[test]
    fn list_set_replaces_code_item() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![BOOL_STACK_ID, BOOL_STACK_ID]));
        test_state.int_stack.push(1);
        test_state.code_stack.push(Item::int(11));
        test_state.code_stack.push(Item::int(22));
        test_state.code_stack.push(Item::int(33));
        list_set(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:Literal(33); 2:List: 1:Literal(true); 2:Literal(false);; 3:Literal(11);",
            "Order of new list element reversed"
        );
    }

    #[test]
    fn list_add_get_preserves_stack_positions() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true;",
            "Initial order of items"
        );
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![BOOL_STACK_ID, BOOL_STACK_ID]));
        list_add(&mut test_state, &icache());
        test_state.int_stack.push(0);
        list_get(&mut test_state, &icache());
        // Run execution
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        assert_eq!(
            PushInterpreter::run(&mut test_state, &mut instruction_set),
            PushInterpreterState::NoErrors
        );
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true;",
            "Push/Pull of list preserves order of items"
        );
    }
    #[test]
    fn list_set_get_preserves_stack_positions() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true;",
            "Initial order of items"
        );
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![BOOL_STACK_ID, BOOL_STACK_ID]));
        test_state.int_stack.push(1);
        test_state.code_stack.push(Item::int(11));
        test_state.code_stack.push(Item::int(22));
        test_state.code_stack.push(Item::int(33));
        list_set(&mut test_state, &icache());
        test_state.int_stack.push(1);
        list_get(&mut test_state, &icache());
        // Run execution
        let mut instruction_set = InstructionSet::new();
        instruction_set.load();
        assert_eq!(
            PushInterpreter::run(&mut test_state, &mut instruction_set),
            PushInterpreterState::NoErrors
        );
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true;",
            "Push/Pull of list preserves order of items"
        );
    }

    #[test]
    fn list_neighbor_ids_pushes_result_for_valid_index() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.5); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(50); // Index
        test_state.int_stack.push(100); // Size
        list_neighbor_ids(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[40,41,50,51,60,61];")
        );
    }

    #[test]
    fn list_neighbor_ids_corrects_out_of_bounds_index() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.5); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(105); // Index
        test_state.int_stack.push(100); // Size
        list_neighbor_ids(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[88,89,98,99];")
        );
        test_state.int_vector_stack.flush();
        test_state.float_stack.push(1.5); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(-10); // Index
        test_state.int_stack.push(100); // Size
        list_neighbor_ids(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[0,1,10,11];")
        );
    }

    #[test]
    fn list_neighbor_ivals_pushes_sort_values() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.0); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(0); // Index
        test_state.int_stack.push(9); // Size
        for i in 10..20 {
            test_state.code_stack.push(litem(i));
        }
        list_neighbor_ids(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[0,1,3];")
        );
        test_state.int_vector_stack.flush();
        test_state.float_stack.push(1.0); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(0); // Index
        test_state.int_stack.push(9); // Size
        test_state.int_stack.push(0); // Position
        list_neighbor_ivals(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[19,18,16];")
        );
    }
}
