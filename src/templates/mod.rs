pub mod counter;
pub mod simple_counter;
pub mod marketplace;

use anyhow::Result;
use std::path::Path;
use crate::utils::DependencyVersions;

pub trait Template {
    #[allow(dead_code)]
    fn generate(&self, project_path: &Path, project_name: &str) -> Result<()>;
    fn generate_with_versions(&self, project_path: &Path, project_name: &str, versions: &DependencyVersions) -> Result<()>;
}