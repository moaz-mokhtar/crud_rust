use crate::{
    db::DbPool,
    error::MyError,
    models::{Driver, NewDriverRequest},
    schema::drivers::dsl::drivers,
};
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

impl Driver {
    pub async fn truncate(pool: &DbPool) -> Result<(), MyError> {
        let mut connection = pool.get()?;
        diesel::delete(drivers).execute(&mut connection)?;
        Ok(())
    }

    pub async fn insert(incoming: NewDriverRequest, pool: &DbPool) -> Result<Driver, MyError> {
        let mut connection = pool.get()?;
        let feedback: Driver = diesel::insert_into(crate::schema::drivers::dsl::drivers)
            .values(&incoming.as_driver())
            .get_result(&mut connection)?;

        Ok(feedback)
    }

    pub async fn get_all(pool: &DbPool) -> Result<Vec<Driver>, MyError> {
        let mut connection = pool.get()?;
        let drivers_list = drivers.load::<Driver>(&mut connection)?;
        Ok(drivers_list)
    }

    pub async fn get_by_id(id: Uuid, pool: &DbPool) -> Result<Driver, MyError> {
        let mut connection = pool.get()?;
        let driver = drivers.find(id).first(&mut connection)?;

        Ok(driver)
    }

    pub async fn delete(id: Uuid, pool: &DbPool) -> Result<(), MyError> {
        let mut connection = pool.get()?;
        let feedback = diesel::delete(drivers.find(id)).execute(&mut connection)?;

        if feedback > 0 {
            Ok(())
        } else {
            return Err(MyError::NotFound {
                message: "No driver with given data to delete!".to_string(),
            });
        }
    }
}
