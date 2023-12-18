use crate::extract::webpage::Webpage;

/// Data extract from a webpage.
#[derive(Clone, Debug)]
pub struct Extract<T> {
    pub webpage: Webpage,
    pub data: T,
}
