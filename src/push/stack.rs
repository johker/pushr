pub struct PushStack<T> {
    elements: Vec<T>,
}

impl<T> PushStack<T>
where
    T: Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
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
        if req_size > self.elements.len() {
            None
        } else {
            let mut dst = Vec::new();
            dst.copy_from_slice(
                &self.elements[self.elements.len() - req_size..self.elements.len()],
            );
            Some(dst)
        }
    }

    pub fn push_vec(&mut self, to_push: &mut Vec<T>) {
        to_push.reverse();
        self.elements.extend(to_push.clone());
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
        let mut test_vec = vec![5, 4];
        test_stack.push_vec(&mut test_vec);
        assert_eq!(test_stack.elements, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn observe_vec() {
        let mut test_stack = PushStack {
            elements: vec![1, 2, 3],
        };

        match test_stack.observe_vec(2) {
            None => assert!(false),
            Some(cv) => {
                assert_eq!(cv[0], 2);
                assert_eq!(cv[1], 3);
            }
        }
        assert_eq!(test_stack.elements.len(), 3)
    }
}
