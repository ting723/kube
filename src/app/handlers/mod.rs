//! Handler modules for different app modes
//!
//! Each handler focuses on a specific mode's keyboard event processing

pub mod list_mode;
pub mod detail_mode;
pub mod logs_mode;
pub mod batch;

// Re-export handler functions
pub use list_mode::handle_list_mode_key;
pub use detail_mode::handle_detail_mode_key;
pub use logs_mode::handle_logs_mode_key;
pub use batch::handle_batch_mode_key;