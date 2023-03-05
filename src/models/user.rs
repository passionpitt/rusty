#[derive(sqlx::FromRow, Debug)]
#[sqlx(transparent)]
pub struct Users {
    pub id: i64,
    pub name: String,
    pub username: String
}
