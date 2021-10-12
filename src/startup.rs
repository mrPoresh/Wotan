use actix_web::{

    dev::Server,
    web::{self, Data},
    App, HttpServer,

};


pub struct Application {

    port: u16,
    server: Server,

}

pub struct ApplicationBaseUrl(pub String);

impl Application {

    pub async fn build() -> std::io::Result<()> {

        run();

        Ok(())

    }

}

pub fn run() -> std::io::Result<()> {

    Ok(())

}