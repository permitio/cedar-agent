use thiserror::Error;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PolicyStoreError {
    /// Reference to PolicySetError.
    #[error("Unable to modify the policies: {0}")]
    PolicySetError(#[from] cedar_policy::PolicySetError),
    /// Policy with the given id was not found.
    #[error("Unable to find policy with id {0}")]
    PolicyNotFoundError(String),
}
