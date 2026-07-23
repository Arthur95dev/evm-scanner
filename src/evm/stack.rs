#[derive(Debug, PartialEq)]
pub enum StackError {
    Underflow,
}

pub type StackValue = [u8; 32];

pub struct Stack {
    values: Vec<StackValue>,
}

impl Stack {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn push(&mut self, value: StackValue) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> Result<StackValue, StackError> {
        self.values.pop().ok_or(StackError::Underflow)
    }

    pub fn peek(&self) -> Option<&StackValue>{
        self.values.last()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&StackValue> {
        self.values.get(index)
    }
}

fn value_from(byte: u8) -> StackValue {
    let mut value = [0u8; 32];
    value[31] = byte;
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack_is_empty() {
        let stack = Stack::new();

        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.peek(), None);
        assert_eq!(stack.get(0), None);
    }

    #[test]
    fn test_push_increases_len() {
        let mut stack = Stack::new();

        stack.push(value_from(1));
        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());

        stack.push(value_from(2));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_pop_returns_last_pushed_value() {
        let mut stack = Stack::new();

        stack.push(value_from(1));
        stack.push(value_from(2));

        assert_eq!(stack.pop(), Ok(value_from(2)));
        assert_eq!(stack.pop(), Ok(value_from(1)));
        assert_eq!(stack.pop(), Err(StackError::Underflow));
    }

    #[test]
    fn test_pop_on_empty_stack_returns_underflow_error() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), Err(StackError::Underflow));
    }

    #[test]
    fn test_peek_returns_last_value_without_removing_it() {
        let mut stack = Stack::new();

        stack.push(value_from(1));
        stack.push(value_from(2));

        assert_eq!(stack.peek(), Some(&value_from(2)));
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_get_returns_value_at_index() {
        let mut stack = Stack::new();

        stack.push(value_from(1));
        stack.push(value_from(2));

        assert_eq!(stack.get(0), Some(&value_from(1)));
        assert_eq!(stack.get(1), Some(&value_from(2)));
        assert_eq!(stack.get(2), None);
    }

    #[test]
    fn test_is_empty_after_pushing_and_popping_all_values() {
        let mut stack = Stack::new();

        stack.push(value_from(1));
        let _ = stack.pop();

        assert!(stack.is_empty());
    }
}