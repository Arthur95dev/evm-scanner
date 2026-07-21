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

    pub fn pop(&mut self) -> Option<StackValue>{
        self.values.pop()
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