use crate::entities::{List, ListItem};
use crate::error::ListError;
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_lists_by_owner(client: &Client, owner: String) -> Result<Vec<List>, ListError> {
    let stmt = "SELECT * FROM list WHERE list.list_owner = $1;";
    let stmt = client.prepare(&stmt).await.unwrap();
    Ok(client
        .query(&stmt, &[&owner])
        .await?
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect()
    )
}

pub async fn create_new_list(client: &Client, new_list: List) -> Result<List, ListError> {
    let stmt = "INSERT INTO list (list_title, list_owner) values ($1, $2) RETURNING $table_fields;";
    let stmt = stmt.replace("$table_fields", &List::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();
    client
        .query(&stmt, &[&new_list.list_title, &new_list.list_owner])
        .await?
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>()
        .pop()
        .ok_or(ListError::NotFound)
}

pub async fn get_list_items(client: &Client, list_id: i32) -> Result<Vec<ListItem>, ListError> {
    let stmt = "SELECT * FROM list_item WHERE list_item.list_id = $1;";
    let stmt = client.prepare(&stmt).await.unwrap();
    let list_items = client
        .query(&stmt, &[&list_id])
        .await?
        .iter()
        .map(|row| ListItem::from_row_ref(row).unwrap())
        .collect();
    Ok(list_items)
}

pub async fn create_list_item(client: &Client, new_item: ListItem) -> Result<ListItem, ListError> {
    let stmt = "INSERT INTO list_item (list_item_description, list_item_status, list_id) values ($1, $2, $3) RETURNING $table_fields;";
    let stmt = stmt.replace("$table_fields", &ListItem::sql_table_fields());
    let stmt = client.prepare(&stmt).await.unwrap();
    client
        .query(&stmt, &[&new_item.list_item_description, &new_item.list_item_status, &new_item.list_id])
        .await?
        .iter()
        .map(|row| ListItem::from_row_ref(row).unwrap())
        .collect::<Vec<ListItem>>()
        .pop()
        .ok_or(ListError::NotFound)
}