use derive_more::{Display, From};
use tokio_postgres::error::Error as PGError;
use deadpool_postgres::PoolError;
use actix_web::ResponseError;

#[derive(Display, Debug, From)]
pub enum ListError {
    NotFound,
    PGError(PGError),
    PoolError(PoolError),
}

impl std::error::Error for ListError {}

impl ResponseError for ListError {}