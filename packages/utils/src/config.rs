use std::path::{Path, PathBuf};

#[allow(unused)]
pub fn get_config_file_path(dir: &Path, config_path: &Path) -> (PathBuf, PathBuf) {
    let root_dir = dir
        .ancestors()
        .find(|a| a.join(config_path).exists())
        .unwrap_or_else(|| {
            eprintln!(
                "{} not found in current directory or ancestors, current_dir is {}",
                config_path.display(),
                dir.display()
            );
            std::process::exit(1);
        });

    // if we got here we found root_dir so config file should exist so we could theoretically unwrap safely
    let config_file_uncanonicalized = root_dir.join(config_path);
    let config_file = config_file_uncanonicalized
        .canonicalize()
        .unwrap_or_else(|_e| {
            eprintln!(
                "Could not find canonical path of {}",
                config_file_uncanonicalized.display()
            );

            std::process::exit(1);
        });

    (root_dir.to_path_buf(), config_file)
}

