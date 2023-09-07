#![deny(deprecated, clippy::all, clippy::pedantic)]

use therror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[deprecated]
    #[error("...")]
    Deprecated,
}
