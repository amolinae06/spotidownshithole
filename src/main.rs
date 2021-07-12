#[macro_use] extern crate log;

mod downloader;
mod error;
mod spotify;
mod ui;
mod tag;
mod converter;

fn main() {
    std::env::set_var("RUST_LOG", "spotidown=debug,warn");
    pretty_env_logger::init();

    ui::start_ui().unwrap();
}