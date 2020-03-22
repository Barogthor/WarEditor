#[macro_use] extern crate lazy_static;

use fern::colors::{Color, ColoredLevelConfig};
use log::{debug, info, trace, warn, error};

pub fn init_logging(){
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::new()
                // use builder methods
                .info(Color::Yellow)
                .error(Color::Red)
                .warn(Color::Magenta)
                .trace(Color::White)
                .debug(Color::Blue);
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                // record.level(),
                colors.color(record.level()),
                message
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