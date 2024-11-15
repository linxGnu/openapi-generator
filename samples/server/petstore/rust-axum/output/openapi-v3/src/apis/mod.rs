pub mod default;
pub mod info_repo;
pub mod repo;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Authorization {
    Authorized,
    Forbidden,
}
