mod conf;
mod data;

pub use conf::{all, app_name, init, is_first_run, preference, save};

#[cfg(feature = "database")]
pub use conf::db_path;
