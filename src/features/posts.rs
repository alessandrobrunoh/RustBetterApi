use actix_web::{get, web, HttpResponse, Responder};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use crate::DbPool;
use crate::models::post::Post;
use crate::schema::posts;

#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, description = "List of posts", body = [Post])
    ),
    tag = "posts"
)]
#[get("/posts")]
async fn posts_handler(pool: web::Data<DbPool>) -> impl Responder {
    // Get a connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    // Use web::block to offload blocking Diesel code to a thread pool
    let results = web::block(move || {
        posts::table
            .limit(5)
            .select(Post::as_select())
            .load::<Post>(&mut *conn)
    })
        .await
        .unwrap()
        .expect("Error loading posts");
    
    HttpResponse::Ok().json(results)
}