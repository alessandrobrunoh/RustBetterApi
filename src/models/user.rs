use diesel::prelude::*;
use serde::Serialize;
use utoipa::gen::serde_json::json;
use utoipa::ToSchema;

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[schema(title = "User Schema", description = "A user in the system")]
#[schema(example = json!({
    "username": "john_doe",
    "email": "jhon_doe@gmail.com"
}))]
#[schema(rename_all = "UPPERCASE")]
pub struct User {
    #[schema(ignore)]
    pub id: i32,
    #[schema(rename = "name")]
    pub username: String,
    pub email: String,
}