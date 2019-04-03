use std::error::Error;
use std::fmt;
use std::io;

use crate::config;
use crate::project_generator;

#[derive(Debug)]
pub enum CompilerError {
  Io(io::Error),
  VibraniumDirectoryNotFound(project_generator::error::ProjectGenerationError),
  InvalidConfig(config::error::ConfigError),
}

impl Error for CompilerError {
  fn description(&self) -> &str {
    match self {
      CompilerError::Io(error) => {
        match error.kind() {
          io::ErrorKind::NotFound => "Couldn't find executable for requested compiler",
          _ => error.description(),
        }
      },
      CompilerError::VibraniumDirectoryNotFound(error) => error.description(),
      CompilerError::InvalidConfig(error) => error.description(),
    }
  }

  fn cause(&self) -> Option<&Error> {
    match self {
      CompilerError::Io(error) => Some(error),
      CompilerError::VibraniumDirectoryNotFound(error) => Some(error),
      CompilerError::InvalidConfig(error) => Some(error),
    }
  }
}

impl fmt::Display for CompilerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      CompilerError::Io(_error) => write!(f, "{}", self.description()),
      CompilerError::VibraniumDirectoryNotFound(error) => write!(f, "{}", error),
      CompilerError::InvalidConfig(error) => write!(f, "{}", error),
    }
  }
}
