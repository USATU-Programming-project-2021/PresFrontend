#![forbid(unsafe_code)]
// #![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use presx::analyzer::generic_analyzer::YmlAnalyzer;
use presx::analyzer::presentation::PresYml;
use presx::analyzer::slide::SlideYml;
use presx::validators::slide;
use std::env;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = env::args().collect();
    match PresYml::from_file(args.get(1).unwrap().to_string()) {
        Ok(pres) => {
            println!("{:?}", pres);
            let app = presx::TemplateApp::new(pres);
            eframe::run_native(Box::new(app));
        }
        Err(err) => println!("{}", err.to_string()),
    }
}
