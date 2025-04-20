use std::{env, fs, path::{Path, PathBuf}, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=content/");

    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let generated_file = root_dir.join("src/generate_typst.rs");
    let mut generated = String::new();

    generated.push_str("#[allow(dead_code)]\npub static CONTENTS: phf::Map<&'static str, &'static str> = phf::phf_map! {");

    for entry in walkdir::WalkDir::new("content").into_iter().filter_map(Result::ok) {
        if !entry.file_type().is_file() { continue }
        
        let file = entry.path().to_str().unwrap();
        let relative: &Path = Path::new(file).strip_prefix("content").unwrap().as_ref();
        let out_file = Path::new(&root_dir).join(format!("assets/{file}")).with_extension("html");

        let output = Command::new("typst")
            .args(["compile", "--features", "html", "--format", "html"])
            .arg(file)
            .arg("-")
            .output()
            .expect("failed to execute typst command");
        let output = String::from_utf8_lossy(&output.stdout).to_string();
        let output = output.replace("\r", "");

        Command::new("notify-send")
            .arg(file)
            .arg(&out_file)
            .status()
            .expect("failed to execute typst command");

        generated.push_str(format!("\n    {relative:?} => r######\"{output}\"######,").as_str());
    }
    generated.push_str(format!("\n}};").as_str());
    fs::write(&generated_file, generated).unwrap();
}
