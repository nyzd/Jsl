pub struct Stack {
    pub items: [f64; 255],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: [0.0; 255],
        }
    }

    pub fn push(&mut self, num: f64) -> &Self {
        self.items.rotate_right(1);
        self.items[0] = num;

        self
    }

    pub fn pop(&mut self) -> f64 {
        let result = self.items[0];
        self.items[0] = 0.0;
        self.items.rotate_left(1);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn push_test() {
        let mut stack = Stack::new();
        stack.push(1.0);
        stack.push(2.0);
        stack.push(3.0);

        assert_eq!(stack.items[0], 3.0);
    }
}