use std::fmt;

pub struct PushStack<T> {
    elements: Vec<T>,
}

impl<T> PushStack<T>
where
    T: Clone + Copy + fmt::Display,
{
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn to_string(&self) -> String {
        println!("Printing stack");
        let mut result = "".to_string();
        for (i, x) in self.elements.iter().rev().enumerate() {
            result.push_str(&format!("{}:{}; ", (i + 1), x));
        }
        result
    }

    pub fn size(&self) -> usize {
        return self.elements.len();
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
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
}
