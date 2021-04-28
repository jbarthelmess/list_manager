use actix_web::{web, post, get, /*delete, put,*/  HttpResponse};
use crate::entities::{List, ListItem};
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

#[post("/lists")]
pub async fn create_list(db_pool: web::Data<Pool>, req_body: web::Json<List>) -> Result<HttpResponse, ListError> {
    let pg_client: Client = db_pool.get().await.map_err(ListError::PoolError)?;
    let new_list = db::create_new_list(&pg_client, req_body.into_inner()).await?;
    Ok(HttpResponse::Created().json(new_list))
}

#[get("/lists/{list_id}")]
pub async fn get_list_items(db_pool: web::Data<Pool>, web::Path((list_id,)): web::Path<(i32,)>) -> Result<HttpResponse, ListError> {
    let pg_client: Client = db_pool.get().await.map_err(ListError::PoolError)?;
    let list_items = db::get_list_items(&pg_client, list_id).await?;
    Ok(HttpResponse::Ok().json(list_items))
}

#[post("/lists/{list_id}")]
pub async fn create_list_item(db_pool: web::Data<Pool>, web::Path((list_id,)): web::Path<(i32,)>, list_item: web::Json<ListItem>) -> Result<HttpResponse, ListError> {
    let pg_client: Client = db_pool.get().await.map_err(ListError::PoolError)?;
    let mut new_item = list_item.into_inner();
    new_item.list_id = list_id;
    let new_item = db::create_list_item(&pg_client, new_item).await?;
    Ok(HttpResponse::Ok().json(new_item))
}
