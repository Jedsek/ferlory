#[cfg(not(debug_assertions))]
use minify_html::{minify, Cfg};

use std::{
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

fn main() {
    let posts_dir = PathBuf::from("../../posts");
    let generated_path = PathBuf::from("src/generated.rs");

    println!("cargo:rerun-if-changed={}", posts_dir.display());
    
    let mut generated = String::new();
    generated.push_str("#[allow(dead_code)]\npub(crate) static POSTS: phf::Map<&'static str, &'static str> = phf::phf_map! {");

    for entry in WalkDir::new(&posts_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry | entry.file_type().is_file())
    {
        let file_path = entry.path();
        let file_name = get_file_name(file_path, &posts_dir);
        let post_content = get_post(file_path);
       
        generated.push_str(format!("\n    \"{}\" => r######\"{}\"######,", file_name.display(), post_content).as_str());
        notify(&file_name);
    }
    generated.push_str(format!("\n}};").as_str());
    std::fs::write(&generated_path, generated).unwrap();
}

// get post name without extension and prefix
fn get_file_name<'a>(file_path: &'a Path, posts_dir: &'a Path) -> PathBuf {
    PathBuf::from(file_path)
        .with_extension("")
        .strip_prefix(posts_dir)
        .unwrap()
        .to_path_buf()
}

fn get_post(file_path: &Path) -> String {
    // compile typst post
    let output = Command::new("typst")
        .args(["compile", "--features", "html", "--format", "html"])
        .arg(file_path)
        .arg("-")
        .output()
        .expect("failed to execute typst command")
        .stdout;

    // minify html
    #[cfg(not(debug_assertions))]
    let output = minify(&output, &Cfg::new());

    let output = String::from_utf8_lossy(&output).to_string();
    let output = output.replace("\r", "");

    output
}

#[allow(unused)]
fn notify(file_name: &Path) {
    Command::new("notify-send")
        .arg(file_name)
        .spawn()
        .expect("failed to execute typst command");
}
