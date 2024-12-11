use log::{debug, info, error, trace, warn, LevelFilter};
//Customize log format using env_logger
use env_logger::Builder;
use std::io::Write;

fn main() {
    //Initialize env_logger
    //buf：实现Write trait 的对象，通常代表日志信息要写入的目标缓冲区
    // 调用 writeln! 宏向这个缓冲区写入格式化后的日志内容。
    Builder::new().format(|buf, record| {
        //
        writeln!(buf,
                 "[{}] - {} - {}",
                 chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), // Timestamp
                 record.level(),
                 //获取实际要记录的消息内容，作为日志的具体信息部分。
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
