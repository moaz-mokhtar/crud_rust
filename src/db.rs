use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use log::info;
use r2d2::Pool;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbClient;
impl DbClient {
    pub fn get_pool_connection() -> DbPool {
        dotenv().ok();

        #[cfg(not(test))]
        let url = env::var("DATABASE_URL").expect("Couldn't find 'DATABASE_URL' inside .env file");

        #[cfg(test)]
        let url = env::var("DATABASE_URL_TEST")
            .expect("Couldn't find 'DATABASE_URL_TEST' inside .env file");

        info!("DATABASE_URL: {url}");

        let migr = ConnectionManager::<PgConnection>::new(url);
        r2d2::Pool::builder()
            .build(migr)
            .expect("could not build connection pool")
    }

    pub fn get_test_pool() -> DbPool {
        dotenv().ok();

        let url = env::var("DATABASE_URL_TEST")
            .expect("Couldn't find 'DATABASE_URL_TEST' inside .env file");

        info!("DATABASE_URL: {url}");

        let migr = ConnectionManager::<PgConnection>::new(url);
        r2d2::Pool::builder()
            .build(migr)
            .expect("could not build connection pool")
    }
}
