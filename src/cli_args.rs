use structopt::StructOpt;   // StructOpt ---> A struct that is converted from command line arguments


// Struct for App parameters
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "wotan")]
pub(crate) struct Opt {

    #[structopt(short, long, env = "PORT", default_value = "3000")]
    pub port: u16,

    #[structopt(long, env = "HOST", default_value = "localhost")]
    pub host: String,

    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,

    #[structopt(
        long,
        env = "AUTH_SECRET_KEY",
        default_value = "01230123012301230123012301230123"
    )]
    pub auth_secret_key: String,

    // secure_cookie: bool ---> Later

    #[structopt(long, env = "AUTH_DURATION_IN_HOUR", default_value = "12")]
    pub auth_duration_in_hour: u16,

}