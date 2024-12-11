use log::{debug, info, error, trace, warn, LevelFilter};
//Customize log format using env_logger
use env_logger::Builder;
use std::io::Write;

fn main() {
    //Initialize env_logger
    Builder::new().format(|buf, record| {
        writeln!(buf,
                 "[{}] - {} - {}",
                 chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), // Timestamp
                 record.level(),                                  // level of log
                 record.args()
        )
    })
        .filter(None, LevelFilter::Info)//Used to output information from the level of info
        //Cuz in the env_logger, the default output starts from warn or higher
        .init();

    //Log recording
    info!("Application started");
    warn!("A warn occurred");
    error!("An error appears");
}