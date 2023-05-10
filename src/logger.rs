use crate::config;
use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Root};
use log4rs::Config;

pub(crate) fn init(conf: &config::Config) {
    let log_level = conf.log_level.unwrap_or(LevelFilter::Info);
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let config = Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(Root::builder().appender("stderr").build(log_level))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
}
