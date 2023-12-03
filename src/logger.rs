use log::LevelFilter;
use log4rs::{
  config::{Root, Appender}, 
  Config, 
  append::console::ConsoleAppender, 
  encode::pattern::PatternEncoder
};

pub struct MServerLogger;

impl MServerLogger {
  pub fn setup() {
    if log4rs::init_file("log4rs.yaml", Default::default()).is_err() {

      let stdout = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new("{d} - {l} - {m}\n"))).build();
      let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();

        if let Err(e) = log4rs::init_config(config) {
          println!("Error applying default logging configuration: \n\t{:?}", e);
          println!("Server output will likely be lost and not output!");
        }
        else {
          log::warn!("Logger failed to initialize with external config. Reverting to default configuration.");
        }
    } else {
      println!("INFO: Logging initialized with external config.");
    }
  }
}

