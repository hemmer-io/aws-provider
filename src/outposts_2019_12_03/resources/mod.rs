//! Resource modules

pub mod capacity_task;
pub use capacity_task::Capacity_task;
pub mod outpost;
pub use outpost::Outpost;
pub mod outpost_billing_information;
pub use outpost_billing_information::Outpost_billing_information;
pub mod outpost_instance_types;
pub use outpost_instance_types::Outpost_instance_types;
pub mod connection;
pub use connection::Connection;
pub mod site_rack_physical_properties;
pub use site_rack_physical_properties::Site_rack_physical_properties;
pub mod outpost_supported_instance_types;
pub use outpost_supported_instance_types::Outpost_supported_instance_types;
pub mod site_address;
pub use site_address::Site_address;
pub mod order;
pub use order::Order;
pub mod site;
pub use site::Site;
pub mod catalog_item;
pub use catalog_item::Catalog_item;

