mod error;

use domain::use_case;

use error::Error;
pub type Result<T> = core::result::Result<T, Error>;

use clap::{Arg, Command};

const DEFAULT_DATABASE_URL: &'static str = "mysql://root:@127.0.0.1:4000/test?charset=utf8mb4";

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let exit_code = match run_app().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{:?}", err);
            1
        }
    };
    std::process::exit(exit_code);
}

async fn run_app() -> Result<()> {
    let app_name = format!("{}", env!("CARGO_PKG_NAME"));
    let version = format!("{}", env!("CARGO_PKG_VERSION"));
    let app = Command::new(app_name.as_str())
        .version(version.as_str())
        .author("Hiroki Tanaka <support@kumano-te.com>")
        .about("")
        .arg(
            Arg::new("database_url")
                .short('d')
                .long("database")
                .help("TiDB url")
                .takes_value(true)
                .required(false)
                .default_value(DEFAULT_DATABASE_URL),
        );
    let matches = app.get_matches();
    let database_url_str = matches
        .value_of("database_url")
        .unwrap_or(DEFAULT_DATABASE_URL);
    let mut store_connection = store::establish_connection(database_url_str)?;
    use_case::cmd::run_simple_and_trade::execute(&mut store_connection)?;
    Ok(())
}
