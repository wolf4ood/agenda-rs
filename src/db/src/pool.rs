use anyhow::Result;
use diesel::r2d2::{Builder, ConnectionManager, Pool as DieselPool, PooledConnection};
use diesel::{Connection, PgConnection};

embed_migrations!();
pub struct Pool<T: Connection + 'static> {
    pool: DieselPool<ConnectionManager<T>>,
}

impl<T: Connection + 'static> Pool<T> {
    pub fn new(db_url: &str) -> Pool<T> {
        let manager = ConnectionManager::new(db_url);
        let pool = Builder::default()
            .build(manager)
            .expect("could not initiate test db pool");
        Pool { pool }
    }

    pub fn get(&self) -> Result<PooledConnection<ConnectionManager<T>>> {
        self.pool.get().map(Ok)?
    }
}

pub fn drop_and_create_db(db_url: &str, db_name: &str) {
    let pool: Pool<PgConnection> = Pool::new(db_url);

    let conn = pool.get().unwrap();

    conn.execute(&format!("DROP DATABASE IF EXISTS {}", db_name))
        .unwrap();
    conn.execute(&format!("CREATE DATABASE {}", db_name))
        .unwrap();

    let pool: Pool<PgConnection> = Pool::new(format!("{}/{}", db_url, db_name).as_str());

    let conn = pool.get().unwrap();

    embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
}
