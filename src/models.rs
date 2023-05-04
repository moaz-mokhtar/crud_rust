use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::drivers;

#[derive(Debug, Deserialize, Serialize)]
pub struct DefaultResponse {
    pub description: String,
}

#[derive(
    Debug, Clone, Deserialize, Serialize, Queryable, PartialEq, Insertable, PartialOrd, Eq, Ord,
)]
#[diesel(table_name = drivers)]
pub struct Driver {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct DriverDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDriverRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}

impl NewDriverRequest {
    pub fn as_driver(&self) -> Driver {
        Driver {
            id: Uuid::new_v4(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
        }
    }
}

impl Driver {
    pub fn as_dto(&self) -> DriverDTO {
        DriverDTO {
            id: self.id,
            name: format!("{} {}", self.first_name, self.last_name),
            email: self.email.clone(),
            phone: self.phone.clone(),
        }
    }
}
