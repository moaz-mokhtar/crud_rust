use crate::{
    db::DbPool,
    error::MyError,
    models::{Driver, DriverDTO, NewDriverRequest},
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

    pub async fn insert(incoming: NewDriverRequest, pool: &DbPool) -> Result<DriverDTO, MyError> {
        let mut connection = pool.get()?;
        let feedback: Driver = diesel::insert_into(crate::schema::drivers::dsl::drivers)
            .values(&incoming.as_driver())
            .get_result(&mut connection)?;

        Ok(feedback.as_dto())
    }

    pub async fn insert_bulk(incoming: Vec<Driver>, pool: &DbPool) -> Result<usize, MyError> {
        let mut connection = pool.get()?;
        let feedback = diesel::insert_into(crate::schema::drivers::table)
            .values(&incoming)
            .execute(&mut connection)?;

        Ok(feedback)
    }

    pub async fn get_all(pool: &DbPool) -> Result<Vec<DriverDTO>, MyError> {
        let mut connection = pool.get()?;
        let drivers_list = drivers.load::<Driver>(&mut connection)?;
        let drivers_dto: Vec<DriverDTO> = drivers_list
            .into_iter()
            .map(|driver| driver.as_dto())
            .collect();

        Ok(drivers_dto)
    }

    pub async fn get_all_by_name(pool: &DbPool) -> Result<Vec<DriverDTO>, MyError> {
        let mut connection = pool.get()?;
        let drivers_list = drivers.load::<Driver>(&mut connection)?;
        let mut drivers_dto: Vec<DriverDTO> = drivers_list
            .into_iter()
            .map(|driver| driver.as_dto())
            .collect();

        drivers_dto.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(drivers_dto)
    }

    pub async fn get_all_by_char(pool: &DbPool) -> Result<Vec<DriverDTO>, MyError> {
        let mut connection = pool.get()?;
        let drivers_list = drivers.load::<Driver>(&mut connection)?;

        let alphabetized_drivers: Vec<DriverDTO> = drivers_list
            .into_iter()
            .map(|driver| {
                let driver = Driver {
                    id: driver.id,
                    first_name: sort_name_by_char(driver.first_name),
                    last_name: sort_name_by_char(driver.last_name),
                    email: driver.email,
                    phone: driver.phone,
                };
                driver.as_dto()
            })
            .collect();

        Ok(alphabetized_drivers)
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

pub fn sort_name_by_char(name: String) -> String {
    let chars: std::collections::BTreeSet<char> = name.chars().collect();
    chars.into_iter().collect()
}
