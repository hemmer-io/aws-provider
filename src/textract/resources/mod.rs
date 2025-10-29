//! Resource modules

pub mod document_text_detection;
pub use document_text_detection::Document_text_detection;
pub mod lending_analysis_summary;
pub use lending_analysis_summary::Lending_analysis_summary;
pub mod adapter;
pub use adapter::Adapter;
pub mod lending_analysis;
pub use lending_analysis::Lending_analysis;
pub mod document_analysis;
pub use document_analysis::Document_analysis;
pub mod adapter_version;
pub use adapter_version::Adapter_version;
pub mod expense_analysis;
pub use expense_analysis::Expense_analysis;

