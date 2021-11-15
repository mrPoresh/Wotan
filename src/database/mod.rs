use crate::errors::ServiceError;

use diesel::r2d2::PoolError;

pub mod pool;

type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;


pub fn db_connection(pool: &Pool) -> Result<PooledConnection, ServiceError> {

    Ok(pool.get().map_err(|_| ServiceError::UnableToConnectToDb)?)

}