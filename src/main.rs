//Rusty APP
//@author: mindo
mod models;
mod db;

use futures::TryStreamExt;
use models::user::Users;
use db::connection::create_database_connection;
use dotenv::dotenv;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let pool = create_database_connection().await?;

    //fetch record
    let mut query_results = sqlx::query_as::<_, Users>(r#"SELECT id, name, username FROM users ORDER BY id ASC"#)
        .fetch(&pool);

    println!("+-------- Test Result ---------+");
    while let Some(row) = query_results.try_next().await? {
        println!("user id: {:?}, name: {:?}, username: {:?}", row.id, row.name, row.username)
    }

    Ok(())
}
