use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use blockchain_project::storage::Storage;
use reqwest;

mod vm;

pub use vm::VirtualMachine;

pub struct SmartContract {
    pub code: String, // The code of the smart contract
    pub state: HashMap<String, i32>, // The state of the smart contract
    pub roles: HashSet<String>, // Set of roles allowed to execute certain functions
}

impl SmartContract {
    pub fn new(code: String) -> Self {
        SmartContract {
            code,
            state: HashMap::new(),
            roles: HashSet::new(),
        }
    }

    pub fn execute(&mut self, function_name: &str, params: &[i32]) -> Result<i32, Box<dyn Error>> {
        let mut vm = VirtualMachine::new(1000); // Set a default gas limit
        vm.execute_with_gas(&self.code, params, 1000)?;
        match function_name {
            "add" => Ok(params.iter().sum()),
            "multiply" => Ok(params.iter().product()),
            _ => Err("Function not found".into()),
        }
    }

    pub fn save_state(&self, storage: &Storage, contract_id: &str) {
        storage.store_state(contract_id, &self.state);
    }

    pub fn load_state(&mut self, storage: &Storage, contract_id: &str) {
        self.state = storage.load_state(contract_id);
    }

    pub fn emit_event(&self, event_name: &str, data: &str) {
        println!("Event: {} - Data: {}", event_name, data);
        // Add logic to notify listeners or filter events
    }

    pub fn upgrade(&mut self, new_code: String) {
        self.code = new_code;
        // Optionally, handle state migration if needed
    }

    pub fn add_role(&mut self, role: String) {
        self.roles.insert(role);
    }

    pub fn execute_with_role(&mut self, role: &str, function_name: &str, params: &[i32]) -> Result<i32, Box<dyn Error>> {
        if self.roles.contains(role) {
            self.execute(function_name, params)
        } else {
            Err("Access denied".into())
        }
    }

    pub async fn fetch_external_data(&self, url: &str) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(url).await?;
        let data = response.text().await?;
        Ok(data)
    }

    pub fn pause(&mut self) {
        // Logic to pause the contract
    }

    pub fn resume(&mut self) {
        // Logic to resume the contract
    }

    pub fn terminate(&mut self) {
        // Logic to terminate the contract
    }

    pub fn log_execution(&self, message: &str) {
        println!("Contract Execution Log: {}", message);
    }

    pub fn execute_with_error_handling(&mut self, function_name: &str, params: &[i32]) -> Result<i32, Box<dyn Error>> {
        match self.execute(function_name, params) {
            Ok(result) => Ok(result),
            Err(e) => {
                self.log_execution(&format!("Error executing {}: {}", function_name, e));
                Err(e)
            }
        }
    }

    pub fn audit(&self) -> Vec<String> {
        let mut issues = Vec::new();
        if self.code.contains("unsafe") {
            issues.push("Unsafe operations detected".to_string());
        }
        // Add more checks as needed
        issues
    }
}

pub struct EventManager {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&str)>>>,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_name: &str, callback: Box<dyn Fn(&str)>) {
        self.subscribers.entry(event_name.to_string()).or_default().push(callback);
    }

    pub fn emit(&self, event_name: &str, data: &str) {
        if let Some(callbacks) = self.subscribers.get(event_name) {
            for callback in callbacks {
                callback(data);
            }
        }
    }
} 