/// Generic result type for the project, may be updated in the futur to encapsulate more information.
pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;
