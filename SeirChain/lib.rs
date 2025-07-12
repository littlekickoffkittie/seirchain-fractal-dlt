// lib.rs
// Root library file for SeirChain project

pub mod core {
    pub mod triad_matrix {
        pub mod triad_structure;
    }
    pub mod consensus {
        pub mod proof_of_fractal;
        pub mod hierarchical_recursive;
    }
    pub mod security {
        pub mod redundant_paths;
    }
}

pub mod network {
    pub mod routing {
        pub mod multi_path_fractal;
    }
}

pub mod interface {
    pub mod virtual_machine {
        pub mod svm_executor;
    }
    pub mod economics {
        pub mod waclanium_token;
    }
}

// Removed tests module declaration because tests.rs is not in src directory
