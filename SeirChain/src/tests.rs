// unit_tests.rs
// Unit tests for SeirChain core modules

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Core::TriadMatrix::triad_structure::{Triad, Transaction};
    use crate::Core::Consensus::proof_of_fractal::ProofOfFractal;
    use crate::Core::Consensus::hierarchical_recursive::HierarchicalRecursiveConsensus;
    use crate::Core::Security::redundant_paths::RedundantPathSecurity;
    use crate::Network::Routing::multi_path_fractal::MultiPathFractalRouting;
    use crate::Interface::VirtualMachine::svm_executor::SvmExecutor;
    use crate::Interface::Economics::waclanium_token::WaclaniumToken;

    #[test]
    fn test_triad_insert_and_merkle_root() {
        let mut triad = Triad::new();
        let tx = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100,
            timestamp: 1234567890,
        };
        triad.insert_transaction(tx);
        assert_ne!(triad.merkle_root, [0u8; 32]);
    }

    #[test]
    fn test_triad_proof_of_fractal_solve_and_verify() {
        let mut triad = Triad::new();
        let tx = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100,
            timestamp: 1234567890,
        };
        triad.insert_transaction(tx);
        triad.calculate_merkle_root();
        triad.proof_of_fractal_data.difficulty = 1;
        let solved = triad.solve_proof_of_fractal();
        assert!(solved);
        let verified = triad.verify_proof_of_fractal();
        assert!(verified);
    }

    #[test]
    fn test_triad_recursive_consensus() {
        let mut root = Triad::new();
        let child = Triad::new();
        root.add_child(0, child).unwrap();
        root.proof_of_fractal_data.difficulty = 1;
        let consensus_result = root.run_recursive_consensus(1);
        assert!(consensus_result);
    }

    #[test]
    fn test_proof_of_fractal_solve_and_verify() {
        let mut pof = ProofOfFractal::new(4);
        let data = b"test data";
        let solved = pof.solve_puzzle(data);
        assert!(solved);
        assert!(pof.verify_solution(data));
    }

    #[test]
    fn test_hierarchical_recursive_consensus() {
        let nodes = vec!["node1".to_string(), "node2".to_string(), "node3".to_string(), "node4".to_string()];
        let mut hrc = HierarchicalRecursiveConsensus::new(nodes, 1, 4);
        assert!(hrc.run_consensus());
        assert!(hrc.validate_subfractal());
    }

    #[test]
    fn test_redundant_path_security() {
        let mut rpsf = RedundantPathSecurity::new();
        rpsf.add_path("path1".to_string());
        assert!(rpsf.validate_paths());
        rpsf.promote_node("node1".to_string());
        assert!(rpsf.is_node_promoted("node1"));
        assert!(rpsf.remove_path("path1"));
        assert!(rpsf.remove_promoted_node("node1"));
    }

    #[test]
    fn test_multi_path_fractal_routing() {
        let mut mpfr = MultiPathFractalRouting::new();
        mpfr.routing_table.insert("coord1".to_string(), vec!["node1".to_string()]);
        mpfr.update_load("node1".to_string(), 10);
        assert_eq!(mpfr.route_transaction("coord1").unwrap()[0], "node1");
        assert_eq!(mpfr.load_balance().unwrap(), "node1");
        assert!(mpfr.remove_node("coord1"));
        mpfr.clear_routing_table();
        assert!(mpfr.list_nodes().is_empty());
    }

    #[test]
    fn test_svm_executor() {
        let mut svm = SvmExecutor::new();
        let input = b"input data";
        let output = svm.execute_contract("contract1", input).unwrap();
        assert_eq!(output, input);
        assert!(svm.contract_exists("contract1"));
        assert!(svm.remove_contract_state("contract1"));
        svm.clear_contract_states();
        assert!(!svm.contract_exists("contract1"));
    }

    #[test]
    fn test_waclanium_token() {
        let mut wac = WaclaniumToken::new(1000, 2000);
        assert_eq!(wac.get_balance("genesis"), 1000);
        assert!(wac.transfer("genesis", "user1", 100).is_ok());
        assert_eq!(wac.get_balance("user1"), 100);
        assert!(wac.stake("user1", 50).is_ok());
        assert_eq!(wac.get_stake("user1"), 50);
        assert!(wac.unstake("user1", 20).is_ok());
        assert_eq!(wac.get_stake("user1"), 30);
        assert!(wac.mint("user1", 500).is_ok());
        assert_eq!(wac.get_balance("user1"), 580);
        assert!(wac.governance_vote("user1", 10));
    }
}
