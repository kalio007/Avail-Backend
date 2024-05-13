use crate::database::models::{Article, User};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Article>>")]
pub struct FetchUserArticles {
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Article>")]
pub struct CreateArticle {
    pub title: String,
    pub content: String,
    pub created_by: i32,
}
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct DeleteUser{
    pub id: i32
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct UpdateUser {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}