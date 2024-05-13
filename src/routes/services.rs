use actix_web::{get, post, web::{Data,Json, Path}, Responder, HttpResponse};
use serde::Deserialize;
use crate::{database::insertable::{CreateArticle, FetchUser, FetchUserArticles}, lib::db_utils::{AppState, DbActor}};
use actix::Addr;
// use crate::lib::db_utils::{AppState, DbActor};


#[derive(Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await{
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No users found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}
