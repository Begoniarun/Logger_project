# Logger

A **logger** is a tool used in software development to record messages or information about the execution of a program. It’s often used for debugging, monitoring, or analyzing software behavior. Loggers are part of logging frameworks, which allow developers to track what’s happening in their applications.



Step1:

在项目的 Cargo.toml 中添加依赖：

```
[dependencies]
log = "0.4"
env_logger = "0.10"
```

```
use env_logger::Builder;
use log::{info, warn, error};
use std::io::Write;

fn main() {
    // 自定义日志格式
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] - {} - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), // 时间戳
                record.level(),                                  // 日志级别
                record.args()                                    // 日志信息
            )
        })
        .init();

    info!("This is an informational message.");
    warn!("This is a warning message.");
    error!("This is an error message.");
}
```

```
use log::{debug, info, error, trace, warn};
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
        .init();

    //Log recording
    info!("Application started");
    warn!("A warn occurred");
    error!("An error appears");
}
```

