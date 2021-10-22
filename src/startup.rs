use crate::configuration::{DatabaseSettings, Settings};
use crate::handlers::route_config;
use crate::configuration::CryptoService;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use std::sync::Arc;

use actix_web::{

    dev::Server,
    web::Data,
    App, HttpServer,
    middleware::Logger,

};


pub struct Application {

    port: u16,
    server: Server,

}

pub struct ApplicationBaseUrl(pub String);

impl Application {

    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {

        let connection_pool = get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to Postgres.");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let crypto = get_crypto(configuration.crypto.key);

        //println!("-- Secret key is --> {:?}", crypto.key);

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
             connection_pool,
            configuration.application.base_url,
            crypto,
            )?;

        Ok(Self { port, server })

    }

    pub fn port(&self) -> u16 {

        self.port

    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {

        self.server.await
        
    }

}

pub async fn get_connection_pool(
    configuration: &DatabaseSettings
) -> Result<PgPool, sqlx::Error> {

    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.with_db())
        .await

}

pub fn get_crypto(crypto: String) -> CryptoService {

    CryptoService {

        key: Arc::new(crypto.clone())

    }

}

pub fn run(

    listener: TcpListener, 
    db_pool: PgPool, 
    base_url: String,
    crypto: CryptoService,

    ) -> Result<Server, std::io::Error> {

    let db_pool = Data::new(db_pool);
    let crypto = Data::new(crypto);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(db_pool.clone())
            .app_data(crypto.clone())
            .app_data(ApplicationBaseUrl(base_url.clone()))
            .configure(route_config)
    })
    .listen(listener)?
    .run();
    
    Ok(server)

}