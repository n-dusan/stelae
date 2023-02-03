#![allow(
    // derive_more doesn't respect these lints
    clippy::pattern_type_mismatch,
    clippy::use_self
)]

//! Stelae-specific errors

use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::{Display, Error};

/// Collection of possible Stelae errors
#[derive(Debug, Display, Error)]
pub enum StelaeError {
    /// Errors generated by the Git server
    #[display(fmt = "A Git server occurred")]
    GitError,
}

#[allow(clippy::missing_trait_methods)]
impl error::ResponseError for StelaeError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::GitError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
