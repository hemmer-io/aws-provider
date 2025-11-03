//! Resource modules

pub mod keyspace;
pub use keyspace::Keyspace;
pub mod table_auto_scaling_settings;
pub use table_auto_scaling_settings::Table_auto_scaling_settings;
pub mod table;
pub use table::Table;
pub mod type;
pub use type::Type;

