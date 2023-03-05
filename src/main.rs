//Rusty APP
//@author: mindo
use sqlx::mysql::MySqlPoolOptions;
use futures::TryStreamExt;

mod model;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(3)
        .connect("mysql://user:topsecret@localhost/rusty")
        .await?;

    //fetch record
    let mut query_results = sqlx::query_as::<_, model::Users>(r#"SELECT id, name, username FROM users ORDER BY id ASC"#)
        .fetch(&pool);

    println!("+-------- Test Result ---------+");
    while let Some(row) = query_results.try_next().await? {
        println!("user id: {:?}, name: {:?}, username: {:?}", row.id, row.name, row.username)
    }

    Ok(())
}
