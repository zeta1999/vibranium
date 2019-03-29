pub mod blockchain;
pub mod project_generator;
pub mod compiler;
pub mod config;

use std::process::{ExitStatus};
use std::path::PathBuf;

#[macro_use]
extern crate serde_derive;
extern crate glob;

#[derive(Debug)]
pub struct Vibranium {
  project_path: PathBuf,
  config: config::Config,
}

impl Vibranium {
  pub fn new(project_path: PathBuf) -> Vibranium {
    Vibranium {
      config: config::Config::new(project_path.clone()),
      project_path,
    }
  }

  pub fn start_node(&self, config: blockchain::NodeConfig) -> Result<ExitStatus, blockchain::error::NodeError> {
    let node = blockchain::Node::new(config);
    node.start()
        .map(|mut process| process.wait().map_err(blockchain::error::NodeError::Io))
        .and_then(|status| status)
  }

  pub fn init_project(&self) -> Result<(), project_generator::error::ProjectGenerationError> {
    let generator = project_generator::ProjectGenerator::new(&self.config);
    generator.generate_project(&self.project_path)
  }

  pub fn reset_project(&self) -> Result<(), project_generator::error::ProjectGenerationError> {
    let generator = project_generator::ProjectGenerator::new(&self.config);
    generator
      .reset_project(&self.project_path)
      .and_then(|_| generator.generate_project(&self.project_path))
  }

  pub fn compile(&self, config: compiler::CompilerConfig) -> Result<ExitStatus, compiler::error::CompilerError> {
    let compiler = compiler::Compiler::new(&self.config);
    let generator = project_generator::ProjectGenerator::new(&self.config);

    generator
      .check_vibranium_dir_exists().map_err(compiler::error::CompilerError::VibraniumDirectoryNotFound)
      .and_then(|_| compiler.compile(config).map(|mut process| process.wait().map_err(compiler::error::CompilerError::Io)))
      .and_then(|status| status)
  }
}
