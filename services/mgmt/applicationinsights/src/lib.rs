#![doc = "generated by AutoRust 0.1.0"]
#[cfg(feature = "package-2020-02-12")]
mod package_2020_02_12;
#[cfg(feature = "package-2020-02-12")]
pub use package_2020_02_12::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-10-20")]
mod package_2020_10_20;
#[cfg(feature = "package-2020-10-20")]
pub use package_2020_10_20::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2020-10")]
mod package_preview_2020_10;
#[cfg(feature = "package-preview-2020-10")]
pub use package_preview_2020_10::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2020-06-only")]
mod package_preview_2020_06_only;
#[cfg(feature = "package-preview-2020-06-only")]
pub use package_preview_2020_06_only::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2020-06")]
mod package_preview_2020_06;
#[cfg(feature = "package-preview-2020-06")]
pub use package_preview_2020_06::{models, operations, API_VERSION};
#[cfg(feature = "package-preview-2020-02")]
mod package_preview_2020_02;
#[cfg(feature = "package-preview-2020-02")]
pub use package_preview_2020_02::{models, operations, API_VERSION};
#[cfg(feature = "package-2015-05")]
mod package_2015_05;
#[cfg(feature = "package-2015-05")]
pub use package_2015_05::{models, operations, API_VERSION};
#[cfg(feature = "package-2017-10")]
mod package_2017_10;
#[cfg(feature = "package-2017-10")]
pub use package_2017_10::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-06-17-preview")]
mod package_2018_06_17_preview;
#[cfg(feature = "package-2018-06-17-preview")]
pub use package_2018_06_17_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2019-10-17-preview")]
mod package_2019_10_17_preview;
#[cfg(feature = "package-2019-10-17-preview")]
pub use package_2019_10_17_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2018-05-01-preview")]
mod package_2018_05_01_preview;
#[cfg(feature = "package-2018-05-01-preview")]
pub use package_2018_05_01_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-02-02-preview")]
mod package_2020_02_02_preview;
#[cfg(feature = "package-2020-02-02-preview")]
pub use package_2020_02_02_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-03-01-preview")]
mod package_2020_03_01_preview;
#[cfg(feature = "package-2020-03-01-preview")]
pub use package_2020_03_01_preview::{models, operations, API_VERSION};
#[cfg(feature = "package-2020-04")]
mod package_2020_04;
use azure_core::setters;
#[cfg(feature = "package-2020-04")]
pub use package_2020_04::{models, operations, API_VERSION};
pub fn config(
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    token_credential: Box<dyn azure_core::TokenCredential>,
) -> OperationConfigBuilder {
    OperationConfigBuilder {
        api_version: None,
        http_client,
        base_path: None,
        token_credential,
        token_credential_resource: None,
    }
}
pub struct OperationConfigBuilder {
    api_version: Option<String>,
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    base_path: Option<String>,
    token_credential: Box<dyn azure_core::TokenCredential>,
    token_credential_resource: Option<String>,
}
impl OperationConfigBuilder {
    setters! { api_version : String => Some (api_version) , base_path : String => Some (base_path) , token_credential_resource : String => Some (token_credential_resource) , }
    pub fn build(self) -> OperationConfig {
        OperationConfig {
            api_version: self.api_version.unwrap_or(API_VERSION.to_owned()),
            http_client: self.http_client,
            base_path: self.base_path.unwrap_or("https://management.azure.com".to_owned()),
            token_credential: Some(self.token_credential),
            token_credential_resource: self.token_credential_resource.unwrap_or("https://management.azure.com/".to_owned()),
        }
    }
}
pub struct OperationConfig {
    api_version: String,
    http_client: std::sync::Arc<std::boxed::Box<dyn azure_core::HttpClient>>,
    base_path: String,
    token_credential: Option<Box<dyn azure_core::TokenCredential>>,
    token_credential_resource: String,
}
impl OperationConfig {
    pub fn api_version(&self) -> &str {
        self.api_version.as_str()
    }
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.http_client.as_ref().as_ref()
    }
    pub fn base_path(&self) -> &str {
        self.base_path.as_str()
    }
    pub fn token_credential(&self) -> Option<&dyn azure_core::TokenCredential> {
        self.token_credential.as_deref()
    }
    pub fn token_credential_resource(&self) -> &str {
        self.token_credential_resource.as_str()
    }
}