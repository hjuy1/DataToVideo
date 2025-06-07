pub mod color;
pub mod constants;
pub mod imageproc;
pub mod video;

pub use {constants::*, video::slide};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
