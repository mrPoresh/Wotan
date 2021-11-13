#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod cli_args;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Loads the .env file
    dotenv::dotenv().ok();
    // Initializes the global logger with an env logger
    env_logger::init();

    info!("Wotan is starting now ---> GoodLuck!");

    // Set configuration
    let configuration = {

        use structopt::StructOpt;
        cli_args::Opt::from_args()

    };

    Ok(())

}