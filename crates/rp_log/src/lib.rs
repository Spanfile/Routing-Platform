use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};
pub use log::{debug, error, info, trace, warn};

pub fn setup_logging() -> anyhow::Result<()> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Magenta)
        .warn(Color::Yellow)
        .error(Color::Red);
    // let time_format = "%Y-%m-%d %H:%M:%S";

    Dispatch::new()
        .format(move |out, msg, record| {
            out.finish(format_args!(
                "[{}] {}",
                // "[{} UTC] [{}] {}",
                // chrono::Utc::now().format(time_format),
                colors.color(record.level()),
                msg
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
