use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}

pub fn get_connection(pool: &PgPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().expect("Failed to get database connection from pool")
}

