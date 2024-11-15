pub mod default;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Authorization {
    Authorized,
    Forbidden,
}
