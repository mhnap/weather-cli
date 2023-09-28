use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Got a bad response from the provider API")]
    BadResponse,

    #[error("Internal local storage error")]
    Confy(#[from] confy::ConfyError),

    #[error("Failed to communicate with provider API")]
    Reqwest(#[from] reqwest::Error),
}
