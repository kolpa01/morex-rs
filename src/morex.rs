use crate::config;

pub mod economy;
pub mod account;

pub fn get_version() -> String {
    format!("Version {} | {}", &config::VERSION_NUMBER, &config::VERSION_TEXT)
}
