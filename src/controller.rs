use actix_web::{web, /*post,*/ get, /*delete, put,*/  HttpResponse};
//use crate::entities::{List, ListItem};
//use serde::Deserialize;
use deadpool_postgres::{Client, Pool};
use crate::error::ListError;
use crate::db;

#[get("/lists/{owner}")]
pub async fn get_lists(db_pool: web::Data<Pool>, web::Path((owner,)): web::Path<(String,)>) -> Result<HttpResponse, ListError> {
    let pg_client: Client = db_pool.get().await.map_err(ListError::PoolError)?;
    let all_lists = db::get_lists_by_owner(&pg_client, owner).await?;
    Ok(HttpResponse::Ok().json(all_lists))
}