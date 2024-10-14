use std::str::FromStr;

use super::Profile;
use config::ConfigError;

pub fn get_env_source(prefix: &str) -> config::Environment {
  config::Environment::with_prefix(prefix)
    .prefix_separator("__")
    .separator("__")
}

pub fn get_profile() -> Result<Profile, config::ConfigError> {
  std::env::var("APP__PROFILE")
    .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
    .unwrap_or_else(|_e| Ok(Profile::Dev))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_get_env_source() {
    let test = get_env_source("APP");
    println!("{:?}", test);
  }

  #[test]
  pub fn test_get_profile() {
    let profile = get_profile().unwrap();
    println!("{:?}", profile);
  }
}