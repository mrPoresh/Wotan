use super::{ConnectionManager, Pool, PoolError};

fn init_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder().build(manager)
}

pub(crate) fn establish_connection(cfg: crate::cli_args::Opt) -> Pool {
    init_pool(&cfg.database_url).expect("Failed to create pool")
}