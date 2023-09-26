use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Internal local storage error")]
    Storage(#[from] confy::ConfyError),

    #[error("Failed to communicate with provider API")]
    Api(#[from] reqwest::Error),
}
