use crate::{
    database::insertable::{CreateArticle, FetchUser, FetchUserArticles, CreateUser, DeleteUser, UpdateUser},
    lib::db_utils::{AppState, DbActor},
};
use actix::Addr;
use actix_web::{
    get, post,delete,put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde::Deserialize;
// use crate::lib::db_utils::{AppState, DbActor};

#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}
#[derive(Deserialize)]
pub struct CreateUserBody {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
#[derive(Deserialize)]
pub struct UpdateUserBody {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}


#[post("/users")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateUser {
        id: body.id,
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
    }).await
    {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}
#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}
// #[delete("/users/{id}")]
// pub async fn delete_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
//     let id: i32 = path.into_inner();
//     let db: Addr<DbActor> = state.as_ref().db.clone();

//     match db.send(DeleteUser {id}).await {
//         Ok(Ok(_)) => HttpResponse::Ok().json("User deleted successfully"),
//         Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
//         _ => HttpResponse::InternalServerError().json("Failed to delete user"),
//     }
// }
#[get("/users/{id}/articles")]
pub async fn get_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();
    match db.send(FetchUserArticles { user_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No articles for user {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve user articles"),
    }
}
#[post("/users/{id}/articles")]
pub async fn create_user_article(state: Data<AppState>, path: Path<i32>, body: Json<CreateArticleBody>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("POST /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateArticle {
        title: body.title.to_string(),
        content: body.content.to_string(),
        created_by: id
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create article"),
    }
}
#[put("/users/{id}")]
pub async fn update_user(state: Data<AppState>, path: Path<i32>, body: Json<UpdateUserBody>) -> impl Responder {
    let id: i32 = path.into_inner();
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(UpdateUser {
        id,
        first_name: body.first_name.to_string(),
        last_name: body.last_name.to_string(),
    }).await {
        Ok(Ok(user)) => HttpResponse::Ok().json(user),
        Ok(Err(_)) => HttpResponse::NotFound().json("User not found"),
        _ => HttpResponse::InternalServerError().json("Failed to update user"),
    }
}