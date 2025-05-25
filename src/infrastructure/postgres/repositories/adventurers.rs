use std::sync::Arc;

use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
pub struct AdventursrPostgres {
    db_pool: Arc<PgPoolSquad>
}

impl AdventursrPostgres{
    pub fn new(db_pool:Arc<PgPoolSquad>) -> Self {
        Self {db_pool}
    }
}