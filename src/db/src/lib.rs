mod pool;
mod schema;
mod todo;


#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

#[macro_use]
extern  crate diesel_migrations;

use diesel::PgConnection;
pub use todo::TodoRepoDiesel;

pub type PgPool = pool::Pool<PgConnection>;

pub use pool::drop_and_create_db;
