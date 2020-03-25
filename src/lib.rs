#[macro_use] extern crate lazy_static;

use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, info, trace, warn, error};

pub fn format_data(path: &str) -> String{
    format!("resources/datas/{}",path)
}
pub fn format_slk(path: &str) -> String{
    format!("resources/slk/{}",path)
}

pub fn init_logging(){
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                    .info(Color::Yellow)
                    .error(Color::Red)
                    .warn(Color::Magenta)
                    .trace(Color::White)
                    .debug(Color::Blue);
            out.finish(format_args!(
                "{color_line}[{target}][{level}{color_line}]\x1B[0m {message}",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors.get_color(&record.level()).to_fg_str()
                ),
                target = record.target(),
                // record.level(),
                level = colors.color(record.level()),
                message = message
            ))
        })
        // Add blanket level filter -
        // .level(log::LevelFilter::Debug)
        // - and per-module overrides
        // .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log").unwrap())
        // Apply globally
        .apply().unwrap();

}

pub mod globals;
pub mod helpers;
pub mod map_data;
pub mod blp;