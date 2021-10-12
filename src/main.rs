use downloaderserv::configuration::get_configuration;
use downloaderserv::startup::Application;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //let subscriber: =

    let configuration = get_configuration().expect("Failed to read configuration.");

    //let application: = Application::build();

    //application.run_until_stopped().await?;

    Ok(())

}