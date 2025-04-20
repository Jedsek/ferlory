#![allow(unused)]

use std::{path::PathBuf, sync::LazyLock};

struct Config {
    content_dir: PathBuf,
    asset_dir: PathBuf,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| Config {
    content_dir: PathBuf::from("/contents"),
    asset_dir: PathBuf::from("/assets")
});
