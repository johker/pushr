pub struct PushStack<T> {
    elements: Vec<T>,
}

impl<T> PushStack<T>
where
    T: Clone,
{
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
}
