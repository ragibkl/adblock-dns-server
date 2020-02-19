mod file_loader;
mod http_loader;
mod loader_factory;

pub use file_loader::FileLoader;
pub use http_loader::HttpLoader;
pub use loader_factory::get_loader;
