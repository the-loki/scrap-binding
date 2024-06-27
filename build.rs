use cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut config = cbindgen::Config::default();
    config.cpp_compat = true;
    config.include_version = true;
    config.language = cbindgen::Language::C;
    config.includes = vec!["scrap_binding_tools.h".to_owned()];
    config.include_guard = Some("SCRAP_BINDING_H".to_owned());
    config.function.prefix = Some("SCRAP_API".to_owned());

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/scrap_binding.h");
}
