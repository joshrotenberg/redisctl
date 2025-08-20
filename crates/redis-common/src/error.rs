use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisCtlError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),
    
    #[error("Profile error: {0}")]
    Profile(#[from] ProfileError),
    
    #[error("Command routing error: {0}")]
    Routing(#[from] RoutingError),
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Profile '{name}' not found")]
    ProfileNotFound { name: String },
    
    #[error("No default profile set")]
    NoDefaultProfile,
    
    #[error("Config file error: {message}")]
    FileError { message: String },
}

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Profile '{name}' is type '{actual_type}' but command requires '{expected_type}'")]
    TypeMismatch {
        name: String,
        actual_type: String,
        expected_type: String,
    },
    
    #[error("Missing credentials for profile '{name}'")]
    MissingCredentials { name: String },
}

#[derive(Error, Debug)]
pub enum RoutingError {
    #[error("Command '{command}' exists in both cloud and enterprise. Use 'redisctl cloud {command}' or 'redisctl enterprise {command}'")]
    AmbiguousCommand { command: String },
    
    #[error("Command '{command}' not found in {deployment_type}")]
    CommandNotFound {
        command: String,
        deployment_type: String,
    },
    
    #[error("No profile specified and no default profile set. Use --profile or set REDISCTL_PROFILE")]
    NoProfileSpecified,
}