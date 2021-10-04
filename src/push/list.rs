use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::item::PushType;
use crate::push::state::*;
use crate::push::topology::Topology;
use crate::push::vector::IntVector;
use std::collections::HashMap;

/// Integer numbers (that is, numbers without decimal points).
pub fn load_list_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("LIST.ADD"), Instruction::new(list_add));
    map.insert(String::from("LIST.GET"), Instruction::new(list_get));
    map.insert(String::from("LIST.SET"), Instruction::new(list_set));
    map.insert(
        String::from("LIST.NEIGHBORS"),
        Instruction::new(list_neighbors),
    );
}

/// Generates continuous numbering starting from 0.
pub fn generate_id(push_state: &mut PushState) -> i32 {
    push_state.list_uid = (push_state.list_uid + 1) % i32::MAX;
    return push_state.list_uid;
}

/// Extracts id from the list item specified by the list index.
pub fn extract_id(push_state: &mut PushState, list_index: usize) -> Option<i32> {
    if let Some(list) = push_state.code_stack.get(list_index) {
        match list {
            Item::List { items } => match items.copy(0) {
                Some(Item::Literal { push_type }) => match push_type {
                    PushType::Int { val } => return Some(val),
                    _ => return None,
                },
                _ => return None,
            },
            // List is empty but contains ID
            Item::Literal { push_type } => match push_type {
                PushType::Int { val } => return Some(val.clone()),
                _ => return None,
            },
            _ => return None,
        }
    }
    return None;
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
/// It adds an unique id at the end of the items.
pub fn new_list(push_state: &mut PushState) -> Option<Vec<Item>> {
    if let Some(mut items) = load_items(push_state) {
        items.push(Item::int(generate_id(push_state)));
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

/// LIST.GET: Pushes a copy of the items at the given stack position to the execution stack.
/// The index i is taken from the top of the INTEGER stack and min-max corrected.
pub fn list_get(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index), 0) as usize;
        if let Some(list) = push_state.code_stack.copy(list_index) {
            push_state.exec_stack.push(list);
        }
    }
}

/// LIST.ID: Pushes a copy of the list ID at the given stack position. If no integer
/// is found on top of the stack of the list item this instructions acts a NOOP.
/// The stack position is taken from the top of the INTEGER stack and min-max corrected.
pub fn list_id(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let size = push_state.code_stack.size() as i32;
        let list_index = i32::max(i32::min(size - 1, index), 0) as usize;
        if let Some(id) = extract_id(push_state, list_index) {
            push_state.int_stack.push(id);
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
        if let Some(id) = extract_id(push_state, list_index) {
            if let Some(mut items) = load_items(push_state) {
                items.push(Item::int(id));
                let list_item = Item::list(items);
                let _res = push_state.code_stack.replace(list_index, list_item);
            }
        }
    }
}

/// LIST.SORT: Sorts the elements on the list stack based on the second integer item found. (The
/// first integer is the id)
pub fn list_sort(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    // TODO
}

/// LIST.NEIGHBORS: Calculates the neighborhood for a given index element and length. It
/// pushes the indices that are contained in this neighborhood to the INTVECTOR stack.
/// The size, the number of dimensions and index (vector topology) are taken from the INTEGER
/// stack in that order. The radius is taken from the float stack. Distances are calculated using the
/// Eucledian metric. All values are corrected by max-min. If the size of the top element is not a power
/// of the dimensions the smallest hypercube that includes the indices is used to represent the
/// topology, e.g. two dimensions and size = 38 is represented by[7,7]. Neighbor indices that
/// do no exist (e.g. 40) are ignored.
pub fn list_neighbors(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::vector::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn generate_id_resets_to_0_on_overflow() {
        let mut test_state = PushState::new();
        assert_eq!(test_state.list_uid, 0);
        test_state.list_uid = i32::MAX - 2;
        assert_eq!(generate_id(&mut test_state), i32::MAX - 1);
        assert_eq!(generate_id(&mut test_state), 0);
        assert_eq!(generate_id(&mut test_state), 1);
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
            INT_STACK_ID,
            FLOAT_STACK_ID,
            FLOAT_VECTOR_STACK_ID,
            BOOL_VECTOR_STACK_ID,
            INT_VECTOR_STACK_ID,
        ]));
        list_add(&mut test_state, &icache());
        assert_eq!(test_state.code_stack.to_string(), "1:List: 1:Literal(1); 2:Literal([22]); 3:Literal([1]); 4:Literal([3]); 5:Literal(1f); 6:Literal(1); 7:Literal(true);;");
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
            "1:List: 1:Literal(2.3f); 2:Literal(3); 3:Literal(2); 4:Literal(true);;"
        );
    }

    #[test]
    fn list_set_replaces_code_item() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state
            .int_vector_stack
            .push(IntVector::new(vec![BOOL_STACK_ID]));
        test_state.int_stack.push(1);
        test_state.code_stack.push(Item::int(11));
        test_state.code_stack.push(Item::int(22));
        test_state.code_stack.push(Item::int(33));
        list_set(&mut test_state, &icache());
        assert_eq!(
            test_state.code_stack.to_string(),
            "1:Literal(33); 2:List: 1:Literal(22); 2:Literal(true);; 3:Literal(11);",
            "List set should replace content while maintaining list ID"
        );
    }

    #[test]
    fn list_neighbors_pushes_result_for_valid_index() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.5); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(50); // Index
        test_state.int_stack.push(100); // Size
        list_neighbors(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[40,41,50,51,60,61];")
        );
    }

    #[test]
    fn list_neighbors_corrects_out_of_bounds_index() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.5); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(105); // Index
        test_state.int_stack.push(100); // Size
        list_neighbors(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[88,89,98,99];")
        );
        test_state.int_vector_stack.flush();
        test_state.float_stack.push(1.5); // Radius
        test_state.int_stack.push(2); // Dimensions
        test_state.int_stack.push(-10); // Index
        test_state.int_stack.push(100); // Size
        list_neighbors(&mut test_state, &icache());
        assert_eq!(
            test_state.int_vector_stack.to_string(),
            String::from("1:[0,1,10,11];")
        );
    }
}
