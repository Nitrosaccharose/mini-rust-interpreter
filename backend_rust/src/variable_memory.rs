pub struct VariableMemory {
    memory: std::collections::HashMap<String, f64>,
}
impl VariableMemory {
    pub fn new() -> VariableMemory {
        VariableMemory {
            memory: std::collections::HashMap::new(),
        }
    }
    pub fn get(&self, key: &str) -> Option<&f64> {
        self.memory.get(key)
    }
    pub fn set(&mut self, key: &str, value: f64) {
        self.memory.insert(key.to_string(), value);
    }
    pub fn remove(&mut self, key: &str) {
        self.memory.remove(key);
    }
    pub fn clear(&mut self) {
        self.memory.clear();
    }
}
