#![allow(dead_code)]

extern crate bindgen;
extern crate syntex_syntax as syntax;
#[macro_use]
extern crate log;


mod support;

// Unused until we can generate code for tests
//mod test_cmath;
mod test_enum;
mod test_decl;
mod test_extern;
mod test_func;
mod test_struct;
mod test_union;
mod test_builtins;

struct TestLogger;

impl log::Log for TestLogger {
    fn enabled(&self, metadata: &log::LogMetadata) -> bool {
        metadata.level() <= log::LogLevel::Warn
    }

    fn log(&self, record: &log::LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}

impl TestLogger {
    pub fn init() -> Result<(), log::SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(log::LogLevelFilter::Info);
            Box::new(TestLogger)
        })
    }
}
