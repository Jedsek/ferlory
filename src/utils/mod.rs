use dioxus::prelude::*;
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

pub fn launch_app(app: fn() -> Element) {
    dioxus::LaunchBuilder::new()
        // Set the server config only if we are building the server target
        .with_cfg(server_only! {
            ServeConfig::builder()
                // Enable incremental rendering
                .incremental(
                    IncrementalRendererConfig::new()
                        // Store static files in the public directory where other static assets like wasm are stored
                        .static_dir(
                            std::env::current_exe()
                                .unwrap()
                                .parent()
                                .unwrap()
                                .join("public")
                        )
                        // Don't clear the public folder on every build. The public folder has other files including the wasm
                        // binary and static assets required for the app to run
                        .clear_cache(false)
                )
                .enable_out_of_order_streaming()
        })
        .launch(app);
}

#[allow(unused)]
pub fn notify_send(content: &str, timeout_ms: usize) -> document::Eval {
    let js = format!(r#"
let last_timeout_id = null

function notify_send(content, timeout) {{
	if (last_timeout_id !== null) {{
		clearTimeout(last_timeout_id)
	}}
	
	const notification = document.querySelector(".notification");
	const notification_content = document.querySelector(".notification-content");

	notification.style.visibility = "visible"
  notification_content.textContent = content

  last_timeout_id = setTimeout(async () => {{ notification.style.visibility = "hidden" }}, timeout)
}}

addEventListener("copy", (_event) => {{
	notify_send("喂, 文本已经复制好了", 2000)
}});
"#);
    document::eval(r#"
"#)

}
