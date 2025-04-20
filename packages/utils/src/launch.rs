use dioxus::prelude::*;

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


