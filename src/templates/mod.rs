pub mod counter;
pub mod simple_counter;
pub mod marketplace;

use anyhow::Result;
use std::path::Path;
use crate::utils::{DependencyVersions, TemplateVariables};

pub trait Template {
    #[allow(dead_code)]
    fn generate(&self, project_path: &Path, project_name: &str) -> Result<()>;
    #[allow(dead_code)]
    fn generate_with_versions(&self, project_path: &Path, project_name: &str, versions: &DependencyVersions) -> Result<()>;
    fn generate_with_variables(&self, project_path: &Path, variables: &TemplateVariables, versions: &DependencyVersions) -> Result<()>;
}