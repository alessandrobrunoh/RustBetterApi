use actix_web::{get, web, HttpResponse, Responder};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use crate::DbPool;
use crate::models::user::User;
use crate::schema::users;

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "List of users", body = [User])
    ),
    tag = "users")]
#[get("/users")]
async fn users_handler(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let results = web::block(move || {
        users::table
            .limit(5)
            .select(User::as_select())
            .load::<User>(&mut *conn)
    })
        .await
        .unwrap()
        .expect("Error loading posts");
    
    HttpResponse::Ok().json(results)
}