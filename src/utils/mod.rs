//! src/utils/mod.rs

pub mod file_writer;
pub mod http_client;

pub use file_writer::{create_files, create_files_with_properties};
pub use http_client::HttpClient; // TODO: use static resource for the client and use method
// instead
