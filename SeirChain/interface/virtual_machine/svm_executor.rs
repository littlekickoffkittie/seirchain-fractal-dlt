use std::collections::{HashMap, VecDeque};

/// SvmExecutor manages execution and state of smart contracts in SeirChain VM.
pub struct SvmExecutor {
    /// Mapping from contract ID to state bytes.
    pub contract_states: HashMap<String, Vec<u8>>,
    /// Queue of contract executions (contract ID and input data).
    pub execution_queue: VecDeque<(String, Vec<u8>)>,
}

impl SvmExecutor {
    /// Creates a new SvmExecutor instance with empty contract states and execution queue.
    pub fn new() -> Self {
        SvmExecutor {
            contract_states: HashMap::new(),
            execution_queue: VecDeque::new(),
        }
    }

    /// Executes a smart contract with given input, returns output bytes.
    /// Simulates parallel execution by queueing and processing.
    /// Currently, it echoes input as output and updates contract state.
    /// Returns an error if contract_id is empty.
    pub fn execute_contract(&mut self, contract_id: &str, input: &[u8]) -> Result<Vec<u8>, String> {
        if contract_id.is_empty() {
            return Err("Contract ID cannot be empty".to_string());
        }

        // Add to execution queue
        self.execution_queue.push_back((contract_id.to_string(), input.to_vec()));

        // Simulate processing queue (in real implementation, this would be async/parallel)
        while let Some((cid, data)) = self.execution_queue.pop_front() {
            // Placeholder: echo input as output
            self.contract_states.insert(cid, data);
        }

        Ok(input.to_vec())
    }

    /// Retrieves the state of a contract by ID.
    pub fn get_contract_state(&self, contract_id: &str) -> Option<&Vec<u8>> {
        self.contract_states.get(contract_id)
    }

    /// Removes a contract state by ID.
    /// Returns true if the contract state was present and removed.
    pub fn remove_contract_state(&mut self, contract_id: &str) -> bool {
        self.contract_states.remove(contract_id).is_some()
    }

    /// Clears all contract states and execution queue.
    pub fn clear_contract_states(&mut self) {
        self.contract_states.clear();
        self.execution_queue.clear();
    }

    /// Checks if a contract exists by ID.
    pub fn contract_exists(&self, contract_id: &str) -> bool {
        self.contract_states.contains_key(contract_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_svm_executor() {
        let svm = SvmExecutor::new();
        assert!(svm.contract_states.is_empty());
        assert!(svm.execution_queue.is_empty());
    }

    #[test]
    fn test_execute_contract() {
        let mut svm = SvmExecutor::new();
        let contract_id = "contract1";
        let input = b"input data";
        let output = svm.execute_contract(contract_id, input).unwrap();
        assert_eq!(output, input);
        assert_eq!(svm.get_contract_state(contract_id), Some(&input.to_vec()));
    }

    #[test]
    fn test_get_contract_state() {
        let mut svm = SvmExecutor::new();
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input).unwrap();
        assert_eq!(svm.get_contract_state(contract_id), Some(&input.to_vec()));
    }

    #[test]
    fn test_remove_contract_state() {
        let mut svm = SvmExecutor::new();
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input).unwrap();
        assert!(svm.remove_contract_state(contract_id));
        assert!(!svm.remove_contract_state(contract_id));
    }

    #[test]
    fn test_clear_contract_states() {
        let mut svm = SvmExecutor::new();
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input).unwrap();
        svm.clear_contract_states();
        assert!(svm.contract_states.is_empty());
    }

    #[test]
    fn test_contract_exists() {
        let mut svm = SvmExecutor::new();
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input).unwrap();
        assert!(svm.contract_exists(contract_id));
        svm.remove_contract_state(contract_id);
        assert!(!svm.contract_exists(contract_id));
    }
}
