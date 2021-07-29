use std::fmt;

#[derive(Clone, Debug)]
pub struct PushStack<T> {
    elements: Vec<T>,
}

impl<T> PushStack<T>
where
    T: Clone + fmt::Display + PartialEq,
{
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Initializes the stack with the argument. Its last
    /// element becomes the top element of the stack.
    pub fn from_vec(elements: Vec<T>) -> Self {
        Self { elements: elements }
    }

    /// Prints the stack from top to bottom enumerating its elements.
    pub fn to_string(&self) -> String {
        let mut result = "".to_string();
        for (i, x) in self.elements.iter().rev().enumerate() {
            result.push_str(&format!("{}:{}; ", (i + 1), x));
        }
        result.trim().to_string()
    }

    /// Returns the stack size.
    pub fn size(&self) -> usize {
        return self.elements.len();
    }

    /// Returns true if the argument equals the
    /// top element of the stack. Uses the = operator for
    /// comparison (shallow for Items)
    pub fn last_eq(&self, item: &T) -> bool {
        return Some(item) == self.elements.last();
    }

    /// Returns true if the element at stack position i counting
    /// from the top. Uses string representation for comparison
    /// (deep)
    pub fn equal_at(&self, i: usize, el: &T) -> Option<bool> {
        if i > self.size() {
            None
        } else {
            Some(self.elements[self.size() - (i + 1)].to_string() == *el.to_string())
        }
    }

    /// Returns a mutable pointer to the element at the bottom
    /// of the stack.
    pub fn bottom_mut(&mut self) -> Option<&mut T> {
        if self.size() > 0 {
            self.elements.first_mut()
        } else {
            None
        }
    }

    /// Removes all elements from the stack.
    pub fn flush(&mut self) {
        self.elements = Vec::new();
    }

    /// Replace element at position i counting from the top. In case the index does not
    /// exist it returns the offset to the size of the stack wrapped in the Err enum.
    pub fn replace(&mut self, i: usize, new_el: T) -> Result<(), usize> {
        let size = &mut self.size();
        match i.checked_sub(*size) {
            None => {
                let _ = std::mem::replace(&mut self.elements[*size - (i + 1)], new_el);
                Ok(())
            }
            Some(diff) => Err(diff + 1),
        }
    }

    /// Returns a mutable reference to the element at stack position i counting
    /// from the top of the stack
    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        let size = &mut self.size();
        if i < *size {
            Some(&mut self.elements[*size - (i + 1)])
        } else {
            None
        }
    }

    /// Returns a reference to the element at stack position i counting from the
    /// top of the stack
    pub fn get(&self, i: usize) -> Option<&T> {
        let size = self.size();
        if i < size {
            Some(&self.elements[size - (i + 1)])
        } else {
            None
        }
    }

    /// Pushes element to the top of the stack.
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    /// Pushes element to the bottom of the stack.
    pub fn push_front(&mut self, value: T) {
        self.elements.insert(0, value);
    }

    /// Removes an indexed item from stack position i counting from the top and
    /// pushes it on top of the stack.
    pub fn yank(&mut self, index: usize) {
        if index < self.size() {
            let el = self.elements.remove(self.size() - (index + 1));
            self.elements.push(el);
        }
    }

    /// Removes the top element from the stack and inserts it at
    /// position index counting from the top.
    pub fn shove(&mut self, index: usize) {
        if index < self.size() {
            if let Some(el) = self.elements.pop() {
                self.elements.insert(self.size() - index, el);
            }
        }
    }

    /// Removes the top element from the stack and returns it.
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Pops and returns the n top-most elements of the stack.
    /// The last element of the returned vector is the top
    /// element of the stack.
    pub fn pop_vec(&mut self, n: usize) -> Option<Vec<T>> {
        if n > self.elements.len() {
            None
        } else {
            Some(
                self.elements
                    .split_off(self.elements.len() - n)
                    .into_iter()
                    .collect(),
            )
        }
    }

    /// Returns a copy of the element at stack position i counting
    /// from top to bottom.
    pub fn copy(&self, i: usize) -> Option<T> {
        if i > self.size() - 1 {
            None
        } else {
            Some(self.elements[self.size() - (i + 1)].clone())
        }
    }

    /// Returns a copy of the n top-most elements
    /// of the stack. The first element of the returned vector
    /// is the nth element counted from the top of the stack.
    pub fn copy_vec(&self, n: usize) -> Option<Vec<T>> {
        if n > self.size() {
            None
        } else {
            let mut cpy = Vec::with_capacity(n);
            for i in 0..n {
                cpy.push(self.elements[self.size() - n + i].clone());
            }
            Some(cpy)
        }
    }

    /// Pushes the argument to the stack where the last
    /// element of the argument will at the top of the stack.
    pub fn push_vec(&mut self, to_push: Vec<T>) {
        self.elements.extend(to_push);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_vec_in_right_order() {
        let mut test_stack = PushStack {
            elements: vec![1, 2, 3],
        };

        match test_stack.pop_vec(2) {
            None => assert!(false),
            Some(pv) => assert_eq!(pv[0], 2),
        }
    }

    #[test]
    fn push_vec_in_right_order() {
        let mut test_stack = PushStack {
            elements: vec![1, 2, 3],
        };
        let test_vec = vec![4, 5];
        test_stack.push_vec(test_vec);
        assert_eq!(test_stack.elements, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn copy_vec_preserves_stack() {
        let test_stack = PushStack {
            elements: vec![1, 2, 3],
        };

        match test_stack.copy_vec(2) {
            None => assert!(false, "Should return values"),
            Some(cv) => {
                assert_eq!(cv.len(), 2);
                assert_eq!(cv[0], 2);
                assert_eq!(cv[1], 3);
            }
        }
        assert_eq!(
            test_stack.elements.len(),
            3,
            "Test stack should remain the same"
        )
    }

    #[test]
    fn equal_at_checks_equality_at_right_index() {
        let test_stack = PushStack {
            elements: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(test_stack.equal_at(0, &5), Some(true));
        assert_eq!(test_stack.equal_at(3, &2), Some(true));
        assert_eq!(test_stack.equal_at(3, &1), Some(false));
    }

    #[test]
    fn yank_vec_returns_right_order() {
        let mut test_stack = PushStack {
            elements: vec![1, 2, 3, 4, 5],
        };
        let mut test_idx = 1;
        test_stack.yank(test_idx);
        assert_eq!(test_stack.elements, [1, 2, 3, 5, 4]);
        test_idx = 5; // No change
        test_stack.yank(test_idx);
        assert_eq!(test_stack.elements, [1, 2, 3, 5, 4]);
        test_idx = 3;
        test_stack.yank(test_idx);
        assert_eq!(test_stack.elements, [1, 3, 5, 4, 2]);
        test_idx = 0; // No change
        test_stack.yank(test_idx);
        assert_eq!(test_stack.elements, [1, 3, 5, 4, 2]);
    }

    #[test]
    fn shove_vec_returns_right_order() {
        let mut test_stack = PushStack {
            elements: vec![1, 2, 3, 4, 5],
        };
        let mut test_idx = 1;
        test_stack.shove(test_idx);
        assert_eq!(test_stack.elements, [1, 2, 3, 5, 4]);
        test_idx = 5; // No change
        test_stack.shove(test_idx);
        assert_eq!(test_stack.elements, [1, 2, 3, 5, 4]);
        test_idx = 3;
        test_stack.shove(test_idx);
        assert_eq!(test_stack.elements, [1, 4, 2, 3, 5]);
        test_idx = 0; // No change
        test_stack.shove(test_idx);
        assert_eq!(test_stack.elements, [1, 4, 2, 3, 5]);
    }

    #[test]
    fn last_eq_preserves_vector() {
        let test_stack = PushStack {
            elements: vec![1, 2, 3, 4, 5],
        };
        let candidate = 5;
        assert_eq!(test_stack.last_eq(&candidate), true);
        let candidate = 4;
        assert_eq!(test_stack.last_eq(&candidate), false);
        assert_eq!(test_stack.size(), 5);
        let test_stack = PushStack {
            elements: Vec::new(),
        };
        assert_eq!(test_stack.last_eq(&candidate), false);
    }

    #[test]
    fn replace_returns_right_offset() {
        let mut test_stack = PushStack {
            elements: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(test_stack.replace(1, 19), Ok(()));
        assert_eq!(test_stack.replace(5, 19), Err(1));
        assert_eq!(test_stack.replace(6, 19), Err(2));
        assert_eq!(test_stack.replace(4, 19), Ok(()));
        assert_eq!(test_stack.replace(0, 19), Ok(()));
        assert_eq!(test_stack.to_string(), "1:19; 2:19; 3:3; 4:2; 5:19;");
    }
}
