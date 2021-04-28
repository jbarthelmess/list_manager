use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="list")]
pub struct List {
    pub list_id: i32,
    pub list_title: String,
    pub list_owner: String
}

#[derive(Serialize, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table="list_item")]
pub struct ListItem {
    pub list_item_id: i32,
    pub list_item_description: String,
    pub list_item_status: i32,
    pub list_id: i32
}