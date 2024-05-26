use thiserror::Error;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PolicyStoreError {
    /// Reference to PolicySetError.
    #[error("Unable to modify the policies: {0}")]
    PolicySetError(#[from] cedar_policy::PolicySetError),
    /// Reference to ParseErrors.
    #[error("Unable to parse policy: {0}")]
    PolicyParseError(#[from] cedar_policy_core::parser::err::ParseErrors),
    /// Policy with the given id was not found.
    #[error("Unable to find policy with id {0}")]
    PolicyNotFoundError(String),
    /// Validation returned an error.
    #[error("Failed validating policy {0} against the schema: {1}")]
    PolicyInvalid(String, String)
}
