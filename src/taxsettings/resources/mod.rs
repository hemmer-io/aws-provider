//! Resource modules

pub mod tax_registration;
pub use tax_registration::Tax_registration;
pub mod supplemental_tax_registration;
pub use supplemental_tax_registration::Supplemental_tax_registration;
pub mod tax_exemption_types;
pub use tax_exemption_types::Tax_exemption_types;
pub mod tax_inheritance;
pub use tax_inheritance::Tax_inheritance;
pub mod tax_registration_document;
pub use tax_registration_document::Tax_registration_document;
pub mod tax_exemption;
pub use tax_exemption::Tax_exemption;

