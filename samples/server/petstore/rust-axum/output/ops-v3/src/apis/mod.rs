pub mod default;

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum Authorization {
    Authorized,
    Forbidden,
}
