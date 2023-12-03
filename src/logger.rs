use log4rs::config::Deserializers;

const DEFAULT_CONFIG: &'static str = r#"
refresh_rate: 30 seconds
appenders:
  stdout:
    kind: console
  file:
    kind: file
    path: \"log/output.log\"
    encoder:
      pattern: \"{d} - {l} - {m}\n\"

root:
  level: debug
  appenders:
    - stdout
    - file
"#;

pub struct MServerLogger;

impl MServerLogger {
  pub fn setup() {
    // setup log4rs
    match log4rs::init_file("log4rs.yaml", Default::default()) {
      Ok(i_file) => i_file,
      Err(error) => {
        println!("Logger failed to initalize!");
        println!("Error occurred while attempting to utilize init file. Make sure it's in the root directory!: \n\t{}", error.to_string());

        let deserializers = Deserializers::default();
        match log4rs::config::load_config_file(&DEFAULT_CONFIG, deserializers) {
          Ok(config) => {
            if let Err(error) = log4rs::init_config(config) {
              println!("Error applying default logging configuration: \n\t{:?}", error);
              println!("Log output will not be saved or displayed!");
            }
            else {
              println!("***Reverted to default configuration!***");
            }
          },
          Err(error) => {
            println!("Error parsing default logging configuration: \n\t{:?}", error);
            println!("Log output will not be saved or displayed!");
          }
        }
      }
    };
  }
}

