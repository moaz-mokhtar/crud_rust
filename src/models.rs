use diesel::{ Queryable, Insertable };
use serde::{ Deserialize, Serialize };

use crate::schema::drivers;

#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultResponse {
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, PartialEq, Insertable)]
#[diesel(table_name = drivers)]
struct Driver {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDriverRequest {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}