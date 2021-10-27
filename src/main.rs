use downloaderserv::configuration::get_configuration;
use downloaderserv::startup::Application;
use downloaderserv::telemetry::init_subscriber;

use color_eyre::Result;
use tracing::{info, instrument};


#[warn(unused_variables)]
#[actix_web::main]
#[instrument]
async fn main() -> Result<()> {

    //let subscriber = get_subscriber("dws".into(), "into".into());
    //init_subscriber(subscriber);

    init_subscriber();

    info!("*** Server is starting now -> GoodLuck ***");

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;

    info!("*** Server configuration is complite ***");

    application.run_until_stopped().await?;

    Ok(())

}