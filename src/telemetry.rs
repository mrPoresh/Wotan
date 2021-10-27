//use tracing::subscriber::set_global_default;
//use tracing::Subscriber;
//use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
//use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;    //{layer::SubscriberExt, EnvFilter, Registry};


//pub fn get_subscriber(        too much info:(
//    name: String,
//    env_filter: String
//) -> impl Subscriber + Sync + Send {

//    let env_filter = EnvFilter::try_from_default_env()
//        .unwrap_or_else(|_| EnvFilter::new(env_filter));

//    let formatting_layer = BunyanFormattingLayer::new(name,std::io::stdout);
    
//    Registry::default()
//        .with(env_filter)
//        .with(JsonStorageLayer)
//        .with(formatting_layer)
    
//}

//pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    
//    LogTracer::init().expect("Failed to set logger");
//    set_global_default(subscriber).expect("Failed to set subscriber");

//}

pub fn init_subscriber() {

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init()

}