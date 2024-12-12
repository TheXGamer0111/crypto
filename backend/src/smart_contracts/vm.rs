use std::collections::HashMap;

pub struct VirtualMachine {
    pub memory: HashMap<String, i32>,
    pub gas_limit: u64,
    pub gas_used: u64,
}

impl VirtualMachine {
    pub fn new(gas_limit: u64) -> Self {
        VirtualMachine {
            memory: HashMap::new(),
            gas_limit,
            gas_used: 0,
        }
    }

    pub fn execute(&mut self, code: &str, params: &[i32]) -> Result<i32, String> {
        // Example: Parse and execute more complex code
        if code.contains("loop") {
            // Implement loop logic
            self.gas_used += 10; // Example gas cost for loop
        } else if code.contains("if") {
            // Implement conditional logic
            self.gas_used += 5; // Example gas cost for conditionals
        }

        if self.gas_used > self.gas_limit {
            return Err("Gas limit exceeded".to_string());
        }

        Ok(0) // Placeholder return value
    }

    pub fn execute_with_gas(&mut self, code: &str, params: &[i32], gas_limit: u64) -> Result<i32, String> {
        self.gas_limit = gas_limit;
        self.gas_used = 0;
        self.execute(code, params)
    }

    pub fn execute_with_validation(&mut self, code: &str, params: &[i32]) -> Result<i32, String> {
        if params.iter().any(|&x| x < 0) {
            return Err("Invalid input: negative values are not allowed".to_string());
        }
        self.execute(code, params)
    }

    pub fn optimize_execution(&mut self, code: &str, params: &[i32]) -> Result<i32, String> {
        // Implement optimization techniques to reduce gas usage
        self.execute_with_gas(code, params, 1000) // Example gas limit
    }
} 