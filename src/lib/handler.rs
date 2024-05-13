use crate::database::models::{User, Article};
use crate::lib::db_utils::DbActor;
use crate::database::schema::users::{dsl::*, id as user_id};
use crate::database::schema::articles::{dsl::*, id as article_id};
use crate::database::insertable::{FetchUser, FetchUserArticles, CreateArticle, CreateUser, DeleteUser, UpdateUser};
use crate::database::article_insertable::{NewArticle, NewUser};
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Create User: Unable to establish connection");

        let new_user = NewUser {
            id: msg.id,
            first_name: msg.first_name,
            last_name: msg.last_name
        };

        diesel::insert_into(users)
            .values(new_user)
            .returning((
                user_id,
                first_name,
                last_name
            ))
            .get_result::<User>(&mut conn)
    }
}
// impl Handler<DeleteUser> for DbActor {
//     type Result = QueryResult<usize>;

//     fn handle(&mut self, msg: DeleteUser, _ctx: &mut Self::Context) -> Self::Result {
//         let mut conn = self.0.get().expect("Delete User: Unable to establish connection");

//         let res = diesel::delete(users.filter(user_id.eq(msg.id)))
//             .execute(&mut conn)?;
//         Ok(res)
//     }
// }

impl Handler<CreateArticle> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: CreateArticle, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Create User Article: Unable to establish connection");

        let new_article = NewArticle {
            title: msg.title,
            content: msg.content,
            created_by: msg.created_by,
    };

    diesel::insert_into(articles)
        .values(new_article)
        .returning((
            article_id,
            title,
            content,
            created_by,
            created_on.nullable(),
        ))
        .get_result::<Article>(&mut conn)
    }
}
impl Handler<FetchUser> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, _msg: FetchUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch User: Unable to establish connection");
        users.get_results::<User>(&mut conn)
    }
}

impl Handler<FetchUserArticles> for DbActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, msg: FetchUserArticles, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Fetch User Articles: Unable to establish connection");

        articles.filter(created_by.eq(msg.user_id)).get_results::<Article>(&mut conn)
    }
}
impl Handler<UpdateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: UpdateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Update User: Unable to establish connection");

        let updated_first_name = msg.first_name;
        let updated_last_name = msg.last_name;

        let updated_user = diesel::update(users.filter(user_id.eq(msg.id)))
            .set((first_name.eq(updated_first_name), last_name.eq(updated_last_name)))
            .get_result::<User>(&mut conn)?;

        Ok(updated_user)
    }
}