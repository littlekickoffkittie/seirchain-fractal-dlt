# Development Plan for SeirChain Enhancements

## Introduction
_SeirChain is a distributed ledger technology (DLT) project focusing on robust, secure, and scalable blockchain solutions. This plan details the next phase of enhancements in wallet/explorer integration, networking, and database support._

---

## 1. Wallet and Explorer Integration
**Goal:** Seamlessly connect the Wallet with TriadExplorer for transparent wallet operations and user activity tracking.

### Tasks
- [ ] Integrate Wallet with TriadExplorer to log wallet creation and transaction activities
    - Add event logging for wallet operations (e.g., creation, transfer, error)
    - Implement error handling and recovery for Explorer updates
    - Ensure synchronization between wallet state and explorer activities
    - Provide RESTful APIs for querying wallet-related activities from the Explorer
    - Add comprehensive tests covering edge cases and failure modes
    - Review security implications of exposing wallet activities

**Acceptance Criteria:**
- All wallet events are visible in TriadExplorer
- APIs return expected results and handle errors gracefully
- Security review completed and documented

---

## 2. Node Networking and Peer-to-Peer Status
**Goal:** Establish a resilient networking layer for node communication and real-time status updates.

### Tasks
- [ ] Design and implement node networking module
    - Implement peer-to-peer status tracking and communication
    - Decide whether to extend `Network/Routing/multi_path_fractal.rs` or create new modules ([link](./Network/Routing/multi_path_fractal.rs))
    - Define protocols for node discovery, status updates, and message passing (include sequence diagrams if possible)
    - Ensure secure (e.g., TLS) and efficient networking

**Dependencies:**
- Protocol definitions before implementation
- Security review after implementation

**Acceptance Criteria:**
- Nodes discover each other automatically
- Status updates propagate within 2 seconds

---

## 3. Database Integration
**Goal:** Provide persistent, fault-tolerant storage for ledger, wallet, and network state.

### Tasks
- [ ] Select appropriate database technology (e.g., PostgreSQL, RocksDB)
- [ ] Design schema to support fractal ledger structure and wallet data (attach ER diagram)
- [ ] Implement database put/get operations
- [ ] Integrate database with core modules
- [ ] Ensure data consistency and fault tolerance

**Success Metrics:**
- Transactions persist across restarts
- No data loss during simulated failures

---

## Next Steps

| Task                                   | Priority | Owner         | Due Date   | Status     |
|-----------------------------------------|----------|---------------|------------|------------|
| Wallet-Explorer integration             | High     | @username     | YYYY-MM-DD | [ ]        |
| Node networking design                  | High     | @username     | YYYY-MM-DD | [ ]        |
| Database selection & schema             | Medium   | @username     | YYYY-MM-DD | [ ]        |

- [ ] Prioritize tasks based on project roadmap
- [ ] Assign development tasks to team members
- [ ] Set milestones and deadlines
- [ ] Begin with wallet-explorer integration

---

## Feedback & Changelog

_Contributors: Please suggest improvements via PRs or GitHub Issues._

**Changelog:**  
- 2025-07-13: Major restructuring for clarity, priorities, and assignment tracking
