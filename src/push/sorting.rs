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

    pub fn heap_sort(arr: &mut [Item]) {
        if arr.len() <= 1 {
            return; // already sorted
        }

        Sorting::heapify(arr);

        for end in (1..arr.len()).rev() {
            arr.swap(0, end);
            Sorting::move_down(&mut arr[..end], 0);
        }
    }

    /// Convert 'arr' into a max heap.
    fn heapify(arr: &mut [Item]) {
        let last_parent = (arr.len() - 2) / 2;
        for i in (0..=last_parent).rev() {
            Sorting::move_down(arr, i);
        }
    }

    /// Move the element at 'root' down until 'arr' is a max heap again.
    /// This assumes that the subtrees under `root` are valid max heaps already.
    fn move_down(arr: &mut [Item], mut root: usize) {
        let last = arr.len() - 1;
        loop {
            let left = 2 * root + 1;
            if left > last {
                break;
            }
            let right = left + 1;
            let max = if right <= last && Sorting::uvalue(&arr[right]) > Sorting::uvalue(&arr[left])
            {
                right
            } else {
                left
            };

            if Sorting::uvalue(&arr[max]) > Sorting::uvalue(&arr[root]) {
                arr.swap(root, max);
            }
            root = max;
        }
    }

    /// Heap sort with descending order
    //   pub fn desc_heap_sort(push_state: &mut PushState, n: usize) {
    //        // Build heap (rearrange array)
    //        for (int i = n / 2 - 1; i >= 0; i--)
    //            min_heapify(push_state, n, i);
    //   }

    /// Heapifies a subtree rooted with node i which is an index in the list.
    /// It uses a min heap and obtains stack values by list_uvalue for sorting
    /// in descending order.
    //    pub fn min_heapify(push_state: &mut PushState, n: usize, i: usize) {
    //        let mut smallest = i; // Initialize smallest as root
    //        let l = 2 * i + 1;
    //        let r = 2 * i + 2;
    //
    //        // If left child is smaller than root
    //        if l < n && Sorting::list_lvalue(push_state, l) < Sorting::list_lvalue(push_state, smallest)
    //        {
    //            smallest = l;
    //        }
    //
    //        // If right child is smaller than smallest so far
    //        if r < n && Sorting::list_lvalue(push_state, r) < Sorting::list_lvalue(push_state, smallest)
    //        {
    //            smallest = r;
    //        }
    //
    //        // If smallest is not root
    //        if smallest != i {
    //            push_state.code_stack.swap(i, smallest);
    //
    //            // Recursively heapify the affected sub-tree
    //            Sorting::min_heapify(push_state, n,  smallest);
    //        }
    //    }

    /// Extracts the sort value from the list item.
    /// The sort value is defined as the item below the id (stack position 2).
    /// The function returns f32::INFINITY if no list with at least two items is found or
    /// if the second item does not have type INT/FLOAT.
    pub fn uvalue(item: &Item) -> f32 {
        match item {
            Item::List { items } => {
                if items.size() >= 2 {
                    match items.get(1) {
                        Some(Item::Literal { push_type }) => match push_type {
                            PushType::Int { val } => return val.clone() as f32,
                            PushType::Float { val } => return val.clone(),
                            _ => return f32::INFINITY,
                        },
                        _ => return f32::INFINITY,
                    }
                }
            }
            _ => return f32::INFINITY,
        }
        return f32::INFINITY;
    }

    /// Extracts the sort value from the list item.
    /// The sort value is defined as the item below the id (stack position 2).
    /// The function returns f32::NEG_INFINITY if no list with at least two items is found or
    /// if the second item does not have type INT/FLOAT.
    pub fn lvalue(item: &Item) -> f32 {
        match item {
            Item::List { items } => {
                if items.size() >= 2 {
                    match items.get(1) {
                        Some(Item::Literal { push_type }) => match push_type {
                            PushType::Int { val } => return val.clone() as f32,
                            PushType::Float { val } => return val.clone(),
                            _ => return f32::NEG_INFINITY,
                        },
                        _ => return f32::NEG_INFINITY,
                    }
                }
            }
            _ => return f32::NEG_INFINITY,
        }
        return f32::NEG_INFINITY;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

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
        assert_eq!(Sorting::uvalue(&test_array[0]), 2.0);
        assert_eq!(Sorting::uvalue(&test_array[1]), 1.0);
        assert_eq!(Sorting::uvalue(&test_array[2]), 9.0);
        assert_eq!(Sorting::lvalue(&test_array[0]), 2.0);
        assert_eq!(Sorting::lvalue(&test_array[1]), 1.0);
        assert_eq!(Sorting::lvalue(&test_array[2]), 9.0);
    }

    #[test]
    fn extract_sort_value_returns_infinity_when_item_not_found() {
        let test_item = Item::list(vec![Item::int(2)]);
        assert_eq!(Sorting::uvalue(&test_item), f32::INFINITY);
        assert_eq!(Sorting::lvalue(&test_item), f32::NEG_INFINITY);
    }

    #[test]
    fn empty() {
        let mut arr: Vec<Item> = Vec::new();
        Sorting::heap_sort(&mut arr);
        assert_eq!(&arr, &[]);
    }

    #[test]
    fn single_element() {
        let mut arr = vec![litem(1)];
        Sorting::heap_sort(&mut arr);
        assert!(Item::equals(&arr[0], &litem(1)));
    }

    #[test]
    fn sorted_array() {
        let mut arr = vec![litem(1), litem(2), litem(3), litem(4)];
        Sorting::heap_sort(&mut arr);
        assert!(Item::equals(&arr[0], &litem(1)));
        assert!(Item::equals(&arr[1], &litem(2)));
        assert!(Item::equals(&arr[2], &litem(3)));
        assert!(Item::equals(&arr[3], &litem(4)));
    }

    #[test]
    fn unsorted_array() {
        let mut arr = vec![litem(3), litem(4), litem(2), litem(1)];
        Sorting::heap_sort(&mut arr);
        assert!(Item::equals(&arr[0], &litem(1)));
        assert!(Item::equals(&arr[1], &litem(2)));
        assert!(Item::equals(&arr[2], &litem(3)));
        assert!(Item::equals(&arr[3], &litem(4)));
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = vec![litem(3), litem(4), litem(2), litem(1), litem(7)];
        Sorting::heap_sort(&mut arr);
        assert!(Item::equals(&arr[0], &litem(1)));
        assert!(Item::equals(&arr[1], &litem(2)));
        assert!(Item::equals(&arr[2], &litem(3)));
        assert!(Item::equals(&arr[3], &litem(4)));
        assert!(Item::equals(&arr[4], &litem(7)));
    }

    #[test]
    fn repeated_elements() {
        let mut arr = vec![litem(542), litem(542), litem(542), litem(542)];
        Sorting::heap_sort(&mut arr);
        assert!(Item::equals(&arr[0], &litem(542)));
        assert!(Item::equals(&arr[0], &litem(542)));
        assert!(Item::equals(&arr[0], &litem(542)));
        assert!(Item::equals(&arr[0], &litem(542)));
    }
}
