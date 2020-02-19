use crate::service::config::SourceConfig;
use crate::service::core::Loader;
use crate::service::loader::HttpLoader;

pub fn get_loader(config: &SourceConfig) -> Option<Box<dyn Loader>> {
    match config.kind.clone().as_str() {
        "http" => Some(Box::new(HttpLoader {
            url: config.path.clone(),
        })),
        _ => None,
    }
}
