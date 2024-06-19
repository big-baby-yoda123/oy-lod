use std::io;

pub mod requests;
pub use requests::*; // re-export

pub mod responses;
pub use responses::*; // re-export

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
