use std::collections::{HashMap, VecDeque, HashSet};
use std::sync::{Arc, Mutex};
use tokio::task;

/// SvmExecutor manages execution and state of smart contracts in SeirChain VM.
pub struct SvmExecutor {
    /// Mapping from contract ID to state bytes.
    pub contract_states: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    /// Queue of contract executions (contract ID and input data).
    pub execution_queue: Arc<Mutex<VecDeque<(String, Vec<u8>)>>>,
    /// Dependencies between contracts.
    pub dependencies: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    /// Number of shards for parallel execution.
    num_shards: usize,
}

impl SvmExecutor {
    /// Creates a new SvmExecutor instance with a given number of shards.
    pub fn new(num_shards: usize) -> Self {
        SvmExecutor {
            contract_states: Arc::new(Mutex::new(HashMap::new())),
            execution_queue: Arc::new(Mutex::new(VecDeque::new())),
            dependencies: Arc::new(Mutex::new(HashMap::new())),
            num_shards,
        }
    }

    /// Executes a smart contract with given input, returns output bytes.
    /// Simulates parallel execution by queueing and processing.
    /// Returns an error if contract_id is empty.
    pub async fn execute_contract(&mut self, contract_id: &str, input: &[u8], deps: Vec<String>) -> Result<Vec<u8>, String> {
        if contract_id.is_empty() {
            return Err("Contract ID cannot be empty".to_string());
        }

        let shard_id = self.get_shard_id(contract_id);
        println!("Contract {} assigned to shard {}", contract_id, shard_id);

        let states = Arc::clone(&self.contract_states);
        let deps_map = Arc::clone(&self.dependencies);

        // Add dependencies for the current contract
        let mut deps_guard = deps_map.lock().unwrap();
        let entry = deps_guard.entry(contract_id.to_string()).or_insert_with(HashSet::new);
        for dep in deps {
            entry.insert(dep);
        }
        drop(deps_guard);


        let handle = task::spawn(async move {
            // Wait for dependencies to be met
            loop {
                let deps_guard = deps_map.lock().unwrap();
                if let Some(contract_deps) = deps_guard.get(contract_id) {
                    let states_guard = states.lock().unwrap();
                    if contract_deps.iter().all(|dep| states_guard.contains_key(dep)) {
                        break;
                    }
                } else {
                    break;
                }
                drop(deps_guard);
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }


            let mut states_guard = states.lock().unwrap();
            states_guard.insert(contract_id.to_string(), input.to_vec());
            input.to_vec()
        });

        handle.await.map_err(|e| e.to_string())
    }

    fn get_shard_id(&self, contract_id: &str) -> usize {
        // Simple hash-based sharding
        let mut hash = 0;
        for byte in contract_id.as_bytes() {
            hash = (hash << 5).wrapping_add(hash) + (byte as usize);
        }
        hash % self.num_shards
    }

    /// Retrieves the state of a contract by ID.
    pub fn get_contract_state(&self, contract_id: &str) -> Option<Vec<u8>> {
        let states = self.contract_states.lock().unwrap();
        states.get(contract_id).cloned()
    }

    /// Removes a contract state by ID.
    /// Returns true if the contract state was present and removed.
    pub fn remove_contract_state(&mut self, contract_id: &str) -> bool {
        let mut states = self.contract_states.lock().unwrap();
        states.remove(contract_id).is_some()
    }

    /// Clears all contract states and execution queue.
    pub fn clear_contract_states(&mut self) {
        let mut states = self.contract_states.lock().unwrap();
        states.clear();
        let mut queue = self.execution_queue.lock().unwrap();
        queue.clear();
        let mut deps = self.dependencies.lock().unwrap();
        deps.clear();
    }

    /// Checks if a contract exists by ID.
    pub fn contract_exists(&self, contract_id: &str) -> bool {
        let states = self.contract_states.lock().unwrap();
        states.contains_key(contract_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_svm_executor() {
        let svm = SvmExecutor::new(4);
        assert!(svm.contract_states.lock().unwrap().is_empty());
        assert!(svm.execution_queue.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_execute_contract() {
        let mut svm = SvmExecutor::new(4);
        let contract_id = "contract1";
        let input = b"input data";
        let output = svm.execute_contract(contract_id, input, vec![]).await.unwrap();
        assert_eq!(output, input);
        assert_eq!(svm.get_contract_state(contract_id), Some(input.to_vec()));
    }

    #[tokio::test]
    async fn test_get_contract_state() {
        let mut svm = SvmExecutor::new(4);
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input, vec![]).await.unwrap();
        assert_eq!(svm.get_contract_state(contract_id), Some(input.to_vec()));
    }

    #[tokio::test]
    async fn test_remove_contract_state() {
        let mut svm = SvmExecutor::new(4);
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input, vec![]).await.unwrap();
        assert!(svm.remove_contract_state(contract_id));
        assert!(!svm.remove_contract_state(contract_id));
    }

    #[tokio::test]
    async fn test_clear_contract_states() {
        let mut svm = SvmExecutor::new(4);
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input, vec![]).await.unwrap();
        svm.clear_contract_states();
        assert!(svm.contract_states.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_contract_exists() {
        let mut svm = SvmExecutor::new(4);
        let contract_id = "contract1";
        let input = b"input data";
        svm.execute_contract(contract_id, input, vec![]).await.unwrap();
        assert!(svm.contract_exists(contract_id));
        svm.remove_contract_state(contract_id);
        assert!(!svm.contract_exists(contract_id));
    }

    #[tokio::test]
    async fn test_contract_dependencies() {
        let mut svm = SvmExecutor::new(4);
        let contract1 = "contract1".to_string();
        let contract2 = "contract2".to_string();
        let input1 = b"input1";
        let input2 = b"input2";

        let mut handles = vec![];
        let mut svm2 = SvmExecutor::new(4);
        svm2.contract_states = svm.contract_states.clone();
        svm2.dependencies = svm.dependencies.clone();


        handles.push(tokio::spawn(async move {
            svm2.execute_contract(&contract2, input2, vec![contract1.clone()]).await
        }));

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let mut svm3 = SvmExecutor::new(4);
        svm3.contract_states = svm.contract_states.clone();
        svm3.dependencies = svm.dependencies.clone();
        handles.push(tokio::spawn(async move {
            svm3.execute_contract(&contract1, input1, vec![]).await
        }));

        let results = futures::future::join_all(handles).await;
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());

        assert_eq!(svm.get_contract_state(&contract1), Some(input1.to_vec()));
        assert_eq!(svm.get_contract_state(&contract2), Some(input2.to_vec()));
    }
}
