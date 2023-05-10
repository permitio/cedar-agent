use std::fmt;
use std::fmt::{Display, Formatter};

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) struct EmptyError;

impl Display for EmptyError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        return Ok(());
    }
}
