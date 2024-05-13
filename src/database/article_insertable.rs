use crate::database::schema::{articles, users};
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=articles)]
pub struct NewArticle {
  pub title: String,
  pub content: String,
  pub created_by: i32,
}
#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
}
#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct DeleteUser {
  pub id: i32,
}