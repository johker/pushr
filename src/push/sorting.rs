use crate::push::instructions::InstructionCache;
use crate::push::item::{Item, PushType};
use crate::push::state::PushState;

pub struct Sorting {}

impl Sorting {
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

    /// Sorts the array in descending order which results
    /// in a ascending order for the stack. The items with lowest
    /// values are placed at the top of the stack.
    pub fn heap_sort(arr: &mut [Item], pos_default: &bool) {
        if arr.len() <= 1 {
            return; // already sorted
        }

        Sorting::heapify(arr, pos_default);

        for end in (1..arr.len()).rev() {
            arr.swap(0, end);
            Sorting::move_down(&mut arr[..end], 0, pos_default);
        }
    }

    /// Convert 'arr' into a max heap.
    fn heapify(arr: &mut [Item], pos_default: &bool) {
        let last_parent = (arr.len() - 2) / 2;
        for i in (0..=last_parent).rev() {
            Sorting::move_down(arr, i, pos_default);
        }
    }

    /// Move the element at 'root' down until 'arr' is a min heap again.
    /// This assumes that the subtrees under `root` are valid min heaps already.
    fn move_down(arr: &mut [Item], mut root: usize, pos_default: &bool) {
        let last = arr.len() - 1;
        loop {
            let left = 2 * root + 1;
            if left > last {
                break;
            }
            let right = left + 1;
            let max = if right <= last
                && Sorting::value(&arr[right], pos_default)
                    > Sorting::value(&arr[left], pos_default)
            {
                right
            } else {
                left
            };

            if Sorting::value(&arr[max], pos_default) > Sorting::value(&arr[root], pos_default) {
                arr.swap(root, max);
            }
            root = max;
        }
    }

    /// Extracts the sort value from the list item.
    /// The sort value is defined as the item below the id (stack position 2).
    /// The function returns f32::INFINITY if no list with at least two items is found or
    /// if the second item does not have type INT/FLOAT.
    pub fn value(item: &Item, pos_default: &bool) -> f32 {
        let default_value;
        if *pos_default {
            default_value = f32::INFINITY;
        } else {
            default_value = f32::NEG_INFINITY;
        }
        match item {
            Item::List { items } => {
                if items.size() >= 2 {
                    match items.get(1) {
                        Some(Item::Literal { push_type }) => match push_type {
                            PushType::Int { val } => return val.clone() as f32,
                            PushType::Float { val } => return val.clone(),
                            _ => return default_value,
                        },
                        _ => return default_value,
                    }
                }
            }
            _ => return default_value,
        }
        return default_value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a test list entry with the given
    /// value to sort and the id = 0.
    pub fn litem(i: i32) -> Item {
        Item::list(vec![Item::int(i), Item::int(0)])
    }

    #[test]
    fn generate_id_resets_to_0_on_overflow() {
        let mut test_state = PushState::new();
        assert_eq!(test_state.list_uid, 0);
        test_state.list_uid = i32::MAX - 2;
        assert_eq!(Sorting::generate_id(&mut test_state), i32::MAX - 1);
        assert_eq!(Sorting::generate_id(&mut test_state), 0);
        assert_eq!(Sorting::generate_id(&mut test_state), 1);
    }

    #[test]
    fn extract_sort_value_when_available() {
        let test_array = vec![
            Item::list(vec![Item::int(2), Item::int(5)]),
            Item::list(vec![Item::float(1.0), Item::int(4)]),
            Item::list(vec![Item::int(9), Item::int(3)]),
        ];
        let pos_default = true;
        assert_eq!(Sorting::value(&test_array[0], &pos_default), 2.0);
        assert_eq!(Sorting::value(&test_array[1], &pos_default), 1.0);
        assert_eq!(Sorting::value(&test_array[2], &pos_default), 9.0);
        let pos_default = false;
        assert_eq!(Sorting::value(&test_array[0], &pos_default), 2.0);
        assert_eq!(Sorting::value(&test_array[1], &pos_default), 1.0);
        assert_eq!(Sorting::value(&test_array[2], &pos_default), 9.0);
    }

    #[test]
    fn extract_sort_value_returns_infinity_when_item_not_found() {
        let test_item = Item::list(vec![Item::int(2)]);
        let pos_default = true;
        assert_eq!(Sorting::value(&test_item, &pos_default), f32::INFINITY);
        let pos_default = false;
        assert_eq!(Sorting::value(&test_item, &pos_default), f32::NEG_INFINITY);
    }

    #[test]
    fn empty() {
        let mut arr: Vec<Item> = Vec::new();
        let pos_default = true;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert_eq!(&arr, &[]);
    }

    #[test]
    fn single_element() {
        let mut arr = vec![litem(1)];
        let pos_default = true;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert!(Item::equals(&arr[0], &litem(1)));
    }

    #[test]
    fn sorted_array() {
        let mut arr = vec![litem(1), litem(2), litem(3), litem(4)];
        let pos_default = true;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert!(Item::equals(&arr[0], &litem(1)));
        assert!(Item::equals(&arr[1], &litem(2)));
        assert!(Item::equals(&arr[2], &litem(3)));
        assert!(Item::equals(&arr[3], &litem(4)));
    }

    #[test]
    fn unsorted_array() {
        let mut arr = vec![litem(3), litem(4), litem(2), litem(1)];
        let pos_default = true;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert!(Item::equals(&arr[0], &litem(1)));
        assert!(Item::equals(&arr[1], &litem(2)));
        assert!(Item::equals(&arr[2], &litem(3)));
        assert!(Item::equals(&arr[3], &litem(4)));
    }

    #[test]
    fn unsorted_array_with_empty_items() {
        let mut arr = vec![litem(3), Item::int(4), litem(2), litem(1)];
        let pos_default = true;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert!(Item::equals(&arr[0], &litem(1)));
        assert!(Item::equals(&arr[1], &litem(2)));
        assert!(Item::equals(&arr[2], &litem(3)));
        assert!(Item::equals(&arr[3], &Item::int(4)));
        let pos_default = false;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert!(Item::equals(&arr[0], &Item::int(4)));
        assert!(Item::equals(&arr[1], &litem(1)));
        assert!(Item::equals(&arr[2], &litem(2)));
        assert!(Item::equals(&arr[3], &litem(3)));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = vec![litem(3), litem(4), litem(2), litem(1), litem(7)];
        let pos_default = true;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert!(Item::equals(&arr[0], &litem(1)));
        assert!(Item::equals(&arr[1], &litem(2)));
        assert!(Item::equals(&arr[2], &litem(3)));
        assert!(Item::equals(&arr[3], &litem(4)));
        assert!(Item::equals(&arr[4], &litem(7)));
    }

    #[test]
    fn repeated_elements() {
        let mut arr = vec![litem(542), litem(542), litem(542), litem(542)];
        let pos_default = true;
        Sorting::heap_sort(&mut arr, &pos_default);
        assert!(Item::equals(&arr[0], &litem(542)));
        assert!(Item::equals(&arr[0], &litem(542)));
        assert!(Item::equals(&arr[0], &litem(542)));
        assert!(Item::equals(&arr[0], &litem(542)));
    }
}
