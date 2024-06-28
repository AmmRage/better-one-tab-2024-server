use log4rs::filter::{Filter, Response};
use log::{LevelFilter, Record};
use log4rs::Handle;
use chrono::Local;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Root};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct PassThroughFilter {
    level_range_start: LevelFilter,
    level_range_end: LevelFilter,
}

impl PassThroughFilter {
    /// Creates a new `ThresholdFilter` with the specified threshold.
    pub fn new(level_range_start: LevelFilter, level_range_end: LevelFilter) -> PassThroughFilter {
        PassThroughFilter { level_range_start, level_range_end }
    }
}

impl Filter for PassThroughFilter {
    fn filter(&self, record: &Record) -> Response {
        if (record.level() >= self.level_range_start && record.level() <= self.level_range_end) ||
           (record.level() >= self.level_range_end && record.level() <= self.level_range_start) {
            Response::Accept
        } else {
            Response::Reject
        }
    }
}

pub fn setup_logger() -> Result<Handle, Box<dyn std::error::Error>> {
    // 获取当前日期
    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut config_builder = log4rs::config::runtime::ConfigBuilder::default();
    // trace, debug
    {
        let log_file_path = &format!("logs/{}/debug.app.log", date);
        // 配置日志滚动策略
        let size_trigger = SizeTrigger::new(10 * 1024); // 10KB
        let size_roller = FixedWindowRoller::builder()
            .build(&format!("logs/{}/debug.app.rotate.{{}}.log", date), 30)?;
        let size_trigger_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(size_roller));
        // 配置日志附加器
        let size_rolled_appender = RollingFileAppender::builder()
            .append(true)
            .encoder(Box::new(PatternEncoder::new("{d}, {l}, {m}{n}")))
            .build(log_file_path, Box::new(size_trigger_policy))?;
        config_builder = config_builder.appender(
            Appender::builder()
                .filter(
                    Box::new(PassThroughFilter::new(log::LevelFilter::Trace, log::LevelFilter::Debug)
                    )
                )
                .build("debug_rolling_file", Box::new(size_rolled_appender))
        );
    }


    // info
    {
        let log_file_path = &format!("logs/{}/info.app.log", date);
        // 配置日志滚动策略
        let size_trigger = SizeTrigger::new(10 * 1024); // 10KB
        let size_roller = FixedWindowRoller::builder()
            .build(&format!("logs/{}/info.app.rotate.{{}}.log", date), 30)?;
        let size_trigger_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(size_roller));
        // 配置日志附加器
        let size_rolled_appender = RollingFileAppender::builder()
            .append(true)
            .encoder(Box::new(PatternEncoder::new("{d}, {l}, {m}{n}")))
            .build(log_file_path, Box::new(size_trigger_policy))?;
        config_builder = config_builder.appender(
            Appender::builder()
                .filter(
                    Box::new(PassThroughFilter::new(log::LevelFilter::Info, log::LevelFilter::Info)
                    )
                )
                .build("info_rolling_file", Box::new(size_rolled_appender))
        );
    }



    // warn, error
    {
        let log_file_path = &format!("logs/{}/error.app.log", date);
        // 配置日志滚动策略
        let size_trigger = SizeTrigger::new(10 * 1024); // 10KB
        let size_roller = FixedWindowRoller::builder()
            .build(&format!("logs/{}/error.app.rotate.{{}}.log", date), 30)?;
        let size_trigger_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(size_roller));
        // 配置日志附加器
        let size_rolled_appender = RollingFileAppender::builder()
            .append(true)
            .encoder(Box::new(PatternEncoder::new("{d}, {l}, {m}{n}")))
            .build(log_file_path, Box::new(size_trigger_policy))?;
        config_builder = config_builder.appender(
            Appender::builder()
                .filter(
                    Box::new(PassThroughFilter::new(log::LevelFilter::Warn, log::LevelFilter::Error)
                    )
                )
                .build("error_rolling_file", Box::new(size_rolled_appender))
        );
    }

    // 配置日志根节点
    let config =
    config_builder
        .build(
            Root::builder()
                .appender("debug_rolling_file")
                .appender("info_rolling_file")
                .appender("error_rolling_file")
                .build(log::LevelFilter::Trace)
        )?;

    // 初始化log4rs
    let handle = log4rs::init_config(config)?;
    Ok(handle)
}