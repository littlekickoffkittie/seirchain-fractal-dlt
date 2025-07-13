// Define the schema for the database.

// This module defines the column families for the RocksDB database.
// Each column family corresponds to a different type of data stored in the database.

pub const CF_DEFAULT: &str = "default";
pub const CF_TRIADS: &str = "triads";
pub const CF_TRANSACTIONS: &str = "transactions";
pub const CF_WALLETS: &str = "wallets";
