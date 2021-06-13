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

    pub fn from_vec(elements: Vec<T>) -> Self {
        Self { elements: elements }
    }

    pub fn to_string(&self) -> String {
        let mut result = "".to_string();
        for (i, x) in self.elements.iter().rev().enumerate() {
            result.push_str(&format!("{}:{}; ", (i + 1), x));
        }
        result.trim().to_string()
    }

    pub fn size(&self) -> usize {
        return self.elements.len();
    }

    pub fn last_eq(&self, atom: &T) -> bool {
        return Some(atom) == self.elements.last();
    }

    #[allow(dead_code)]
    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.size() > 0 {
            self.elements.last_mut()
        } else {
            None
        }
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        if self.size() > 0 {
            self.elements.first_mut()
        } else {
            None
        }
    }

    pub fn flush(&mut self) {
        self.elements = Vec::new();
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn push_front(&mut self, value: T) {
        self.elements.insert(0, value);
    }

    pub fn yank(&mut self, index: usize) {
        if index < self.size() - 1 {
            let el = self.elements.remove(self.size() - (index + 1));
            self.elements.push(el);
        }
    }

    pub fn shove(&mut self, index: usize) {
        if index < self.size() {
            if let Some(el) = self.elements.pop() {
                self.elements.insert(self.size() - index, el);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn pop_vec(&mut self, req_size: usize) -> Option<Vec<T>> {
        if req_size > self.elements.len() {
            None
        } else {
            Some(
                self.elements
                    .split_off(self.elements.len() - req_size)
                    .into_iter()
                    .rev()
                    .collect(),
            )
        }
    }

    pub fn observe_vec(&self, req_size: usize) -> Option<Vec<T>> {
        if req_size > self.size() {
            None
        } else {
            let mut cpy = Vec::with_capacity(req_size);
            for i in 1..req_size + 1 {
                cpy.push(self.elements[self.size() - i].clone());
            }
            Some(cpy)
        }
    }

    pub fn push_vec(&mut self, mut to_push: Vec<T>) {
        to_push.reverse();
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
            Some(pv) => assert_eq!(pv[0], 3),
        }
    }

    #[test]
    fn push_vec_in_right_order() {
        let mut test_stack = PushStack {
            elements: vec![1, 2, 3],
        };
        let test_vec = vec![5, 4];
        test_stack.push_vec(test_vec);
        assert_eq!(test_stack.elements, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn observe_vec_preserves_stack() {
        let test_stack = PushStack {
            elements: vec![1, 2, 3],
        };

        match test_stack.observe_vec(2) {
            None => assert!(false, "Should return values"),
            Some(cv) => {
                assert_eq!(cv.len(), 2);
                assert_eq!(cv[0], 3);
                assert_eq!(cv[1], 2);
            }
        }
        assert_eq!(
            test_stack.elements.len(),
            3,
            "Test stack should remain the same"
        )
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
}
