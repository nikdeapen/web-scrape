/// An error extracting data from a webpage.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Error {
    Other(String),
}
