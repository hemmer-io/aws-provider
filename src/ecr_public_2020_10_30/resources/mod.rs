//! Resource modules

pub mod registries;
pub use registries::Registries;
pub mod authorization_token;
pub use authorization_token::Authorization_token;
pub mod repository;
pub use repository::Repository;
pub mod repository_policy;
pub use repository_policy::Repository_policy;
pub mod repository_catalog_data;
pub use repository_catalog_data::Repository_catalog_data;
pub mod image_tags;
pub use image_tags::Image_tags;
pub mod repositories;
pub use repositories::Repositories;
pub mod images;
pub use images::Images;
pub mod registry_catalog_data;
pub use registry_catalog_data::Registry_catalog_data;
pub mod image;
pub use image::Image;

