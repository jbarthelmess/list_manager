use crate::entities::List;
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