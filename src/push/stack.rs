pub struct PushStack<T> {
    elements: Vec<T>,
}

impl<T> PushStack<T> {
    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}
