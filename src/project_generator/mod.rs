extern crate toml;

use std::path::{PathBuf};
use std::fs;
use std::io::Write;

pub mod error;

const VIBRANIUM_CONFIG_FILE: &str = "vibranium.toml";
const VIBRANIUM_PROJECT_DIRECTORY: &str = ".vibranium";
const DEFAULT_CONTRACTS_DIRECTORY: &str = "contracts";
const DEFAULT_ARTIFACTS_DIRECTORY: &str = "artifacts";

#[derive(Serialize, Deserialize, Debug)]
struct ProjectConfig {
  artifacts_dir: String,
  smart_contract_sources: Vec<String>,
}

pub struct ProjectGenerator;

impl ProjectGenerator {
  pub fn new() -> ProjectGenerator {
    ProjectGenerator
  }

  pub fn generate_project(&self, project_path: &PathBuf) -> Result<(), error::ProjectGenerationError> {
    if !project_path.exists() {
      return Err(error::ProjectGenerationError::ProjectPathNotFound);
    }

    let config_path = project_path.join(VIBRANIUM_CONFIG_FILE);

    let mut directories_to_create: Vec<String> = vec![VIBRANIUM_PROJECT_DIRECTORY.to_string(), DEFAULT_CONTRACTS_DIRECTORY.to_string()];

    if !config_path.exists() {
      directories_to_create.push(DEFAULT_ARTIFACTS_DIRECTORY.to_string());

      let config = ProjectConfig {
        artifacts_dir: DEFAULT_ARTIFACTS_DIRECTORY.to_string(),
        smart_contract_sources: vec![DEFAULT_CONTRACTS_DIRECTORY.to_string() + "/**"]
      };

      let config_toml = toml::to_string(&config).map_err(error::ProjectGenerationError::ConfigSerialization)?;
      let mut config_file = fs::File::create(config_path).map_err(error::ProjectGenerationError::Io)?;
      config_file.write_all(config_toml.as_bytes()).map_err(error::ProjectGenerationError::Io)?;
    } else {
      let existing_config: ProjectConfig = toml::from_str(&fs::read_to_string(config_path).map_err(error::ProjectGenerationError::Io)?)
        .map_err(error::ProjectGenerationError::ConfigDeserialization)?;
      directories_to_create.push(existing_config.artifacts_dir);
    }

    for directory in directories_to_create {
      let path = project_path.join(directory);
      if !path.exists() {
        fs::create_dir_all(path).map_err(error::ProjectGenerationError::Io)?;
      }
    }
    Ok(())
  }

  pub fn reset_project(&self, project_path: &PathBuf) -> Result<(), error::ProjectGenerationError> {
    let vibranium_project_directory = project_path.join(VIBRANIUM_PROJECT_DIRECTORY);
    let config_path = project_path.join(VIBRANIUM_CONFIG_FILE);

    if !vibranium_project_directory.exists() {
      return Err(error::ProjectGenerationError::VibraniumDirectoryNotFound);
    }

    let _ = fs::remove_dir_all(vibranium_project_directory);
    let _ = fs::remove_dir_all(project_path.join(DEFAULT_ARTIFACTS_DIRECTORY));

    if config_path.exists() {
      let existing_config: ProjectConfig = toml::from_str(&fs::read_to_string(config_path).map_err(error::ProjectGenerationError::Io)?).map_err(error::ProjectGenerationError::ConfigDeserialization)?;
      let _ = fs::remove_dir_all(project_path.join(existing_config.artifacts_dir));
    }
    Ok(())
  }
}
