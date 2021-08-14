pub type Value = f32;

#[derive(Debug, Clone)]
pub struct ValueArray {
    values: Vec<Value>
}

impl ValueArray {
    pub fn new() -> Self {
        Self {
            values: Vec::new()
        }
    }

    pub fn write_array(&mut self, val: Value) -> usize {
        self.values.push(val);
        self.values.len() - 1
    }

    pub fn get(&self, i: usize) -> Value {
        return self.values[i];
    }
}