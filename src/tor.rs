pub mod systemctl;
pub mod torrc;

pub use systemctl::restart_or_start_tor;
pub use torrc::update_torrc;
