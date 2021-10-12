use std::convert::{TryFrom, TryInto};
use sqlx::postgres::{PgSslMode, PgConnectOptions};


pub enum Environment {

    Local,
    Production,

}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {

    pub database: DatabaseSettings,
    pub application: ApplicationSettings,

}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {

    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,

}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {

    pub host: String,
    pub port: u16,
    pub base_url: String,

}

impl Environment {

    pub fn as_str(&self) -> &'static str {

        match self {

            Environment::Local => "local",
            Environment::Production => "production",

        }

    }
    
}

impl TryFrom<String> for Environment {

    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {

        match s.to_lowercase().as_str() {

            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),

            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),

        }

    }
    
}

impl DatabaseSettings {

    pub fn without_db(&self) -> PgConnectOptions {

        let ssl_mode = if self.require_ssl {
            
            PgSslMode::Require
        
        } else {

            PgSslMode::Prefer

        };
    
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
        
        }

    pub fn with_db(&self) -> PgConnectOptions {

        self.without_db().database(&self.database_name)

    }

}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {

    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;
    
    settings.merge(config::Environment::with_prefix("app").separator("__"))?;
    
    settings.try_into()

}