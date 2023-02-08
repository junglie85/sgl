use std::{
    env,
    path::{Path, PathBuf},
};

use once_cell::sync::OnceCell;

use crate::SglError;

static BASE_PATH: OnceCell<PathBuf> = OnceCell::new();

pub fn base_path() -> PathBuf {
    BASE_PATH
        .get_or_init(|| {
            if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") {
                PathBuf::from(dir)
            } else {
                env::current_exe().unwrap().parent().unwrap().to_path_buf()
            }
        })
        .clone()
}

pub fn load_file<P>(path: P) -> Result<Vec<u8>, SglError>
where
    P: AsRef<Path>,
{
    std::fs::read(base_path().join(path)).map_err(|e| SglError::General(e.to_string()))
}
