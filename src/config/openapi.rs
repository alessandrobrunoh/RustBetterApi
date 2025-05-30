use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::features::posts::posts_handler,
        crate::features::users::users_handler,
    ),
    components(
        schemas(
            crate::models::post::Post,
            crate::models::user::User
        )
    ),
    tags(
        (name = "posts", description = "Post management endpoints"),
        (name = "users", description = "User management endpoints")
    ),
    info(
        title = "Rust API",
        version = "0.1.0",
        description = "API Documentation"
    ))]
pub struct ApiDoc;