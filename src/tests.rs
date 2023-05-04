use std::collections::BTreeSet;

use crate::{
    db::DbClient,
    engine::sort_name_by_char,
    models::{Driver, DriverDTO, NewDriverRequest},
};

#[actix_web::test]
async fn test_mocks_new_driver_request() {
    let mocks = NewDriverRequest::mocks(10);
    dbg!(&mocks);
    assert_eq!(10, mocks.len());
}

#[actix_web::test]
async fn test_mocks_driver() {
    let mocks = Driver::mocks(10);
    dbg!(&mocks);
    assert_eq!(10, mocks.len());
}

#[actix_web::test]
async fn test_sort_name_by_char() {
    let name = "oliver".to_string();

    let expected = format!("{}", "eilorv");
    let result = sort_name_by_char(name);
    assert_eq!(result, expected);
}

#[actix_web::test]
async fn test_truncate_driver() {
    let pool = DbClient::get_test_pool();
    let feedback = Driver::truncate(&pool).await;
    assert_eq!(true, feedback.is_ok());
}

#[actix_web::test]
async fn test_insert_driver() {
    let pool = DbClient::get_test_pool();

    // truncate database to make sure no other data is there
    assert!(Driver::truncate(&pool).await.is_ok());

    let mock = NewDriverRequest::mocks(1)[0].clone();

    // Create a NewDriverRequest NewDriverRequest
    let new_driver = NewDriverRequest {
        first_name: mock.first_name.clone(),
        last_name: mock.last_name.clone(),
        email: mock.email.clone(),
        phone: mock.phone.clone(),
    };

    // Insert the driver into the database
    let inserted_driver = Driver::insert(new_driver, &pool).await.unwrap();

    // Check if the driver was inserted successfully
    assert_eq!(
        inserted_driver.name,
        format!("{} {}", mock.first_name, mock.last_name)
    );
    assert_eq!(inserted_driver.email, mock.email);
    assert_eq!(inserted_driver.phone, mock.phone);

    // Truncate db
    assert!(Driver::truncate(&pool).await.is_ok());
}

#[actix_web::test]
async fn test_insert_bulk_drivers() {
    let pool = DbClient::get_test_pool();
    let count = 10;

    // truncate database to make sure no other data is there
    assert!(Driver::truncate(&pool).await.is_ok());

    let mocks = Driver::mocks(10);
    assert_eq!(mocks.len(), count as usize);

    // Insert the driver into the database
    let inserted_driver = Driver::insert_bulk(mocks, &pool).await.unwrap();

    // Check if the driver was inserted successfully
    assert_eq!(inserted_driver, count as usize);

    // Truncate db
    assert!(Driver::truncate(&pool).await.is_ok());
}

#[actix_web::test]
async fn test_get_all() {
    let pool = DbClient::get_test_pool();
    let count = 10;

    // truncate database to make sure no other data is there
    assert!(Driver::truncate(&pool).await.is_ok());

    let drivers_mocks = NewDriverRequest::mocks(count);
    assert_eq!(drivers_mocks.len(), count as usize);

    // Insert the drivers into the database
    for driver in drivers_mocks {
        Driver::insert(driver, &pool).await.unwrap();
    }

    let drivers_list = Driver::get_all(&pool).await.unwrap();
    assert_eq!(drivers_list.len(), count as usize);

    // Truncate db
    assert!(Driver::truncate(&pool).await.is_ok());
}

#[actix_web::test]
async fn test_get_all_by_name() {
    let pool = DbClient::get_test_pool();
    let count = 5;

    // truncate database to make sure no other data is there
    assert!(Driver::truncate(&pool).await.is_ok());

    let drivers_list: Vec<Driver> = Driver::mocks(count);
    assert_eq!(drivers_list.len(), count as usize);

    let mut sorted_dto: Vec<DriverDTO> = drivers_list
        .clone()
        .into_iter()
        .map(|driver| driver.as_dto())
        .collect();
    sorted_dto.sort_by(|a, b| a.name.cmp(&b.name));

    // Insert the drivers into the database
    let inserted_drivers = Driver::insert_bulk(drivers_list.clone(), &pool)
        .await
        .unwrap();
    assert_eq!(inserted_drivers, count as usize);

    let drivers_list_by_db = Driver::get_all_by_name(&pool).await.unwrap();
    assert_eq!(drivers_list_by_db.len(), count as usize);
    assert_eq!(drivers_list_by_db, sorted_dto);

    // Truncate db
    assert!(Driver::truncate(&pool).await.is_ok());
}

#[actix_web::test]
async fn test_get_all_by_char() {
    let pool = DbClient::get_test_pool();
    let count = 5;

    // truncate database to make sure no other data is there
    assert!(Driver::truncate(&pool).await.is_ok());

    let drivers_list = Driver::mocks(count);
    assert_eq!(drivers_list.len(), count as usize);

    // Sort drivers by name
    let expected_list: Vec<DriverDTO> = drivers_list
        .clone()
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

    // Insert the drivers into the database
    let inserted_drivers = Driver::insert_bulk(drivers_list, &pool).await.unwrap();
    assert_eq!(inserted_drivers, count as usize);

    let drivers_list_by_db = Driver::get_all_by_char(&pool).await.unwrap();
    assert_eq!(drivers_list_by_db.len(), count as usize);
    assert_eq!(drivers_list_by_db, expected_list);

    // Truncate db
    assert!(Driver::truncate(&pool).await.is_ok());
}

// test engine function get_by_id
#[actix_web::test]
async fn test_get_by_id() {
    let pool = DbClient::get_test_pool();

    // truncate database to make sure no other data is there
    assert!(Driver::truncate(&pool).await.is_ok());

    let mock = NewDriverRequest::mocks(1)[0].clone();
    let mock_db = Driver::insert(mock.clone(), &pool).await.unwrap();

    let driver = Driver::get_by_id(mock_db.id, &pool).await.unwrap();
    assert_eq!(driver.first_name, mock.first_name);
    assert_eq!(driver.last_name, mock.last_name);
    assert_eq!(driver.email, mock.email);
    assert_eq!(driver.phone, mock.phone);

    // Truncate db
    assert!(Driver::truncate(&pool).await.is_ok());
}

// test for delete function
#[actix_web::test]
async fn test_delete() {
    let pool = DbClient::get_test_pool();

    // truncate database to make sure no other data is there
    assert!(Driver::truncate(&pool).await.is_ok());

    let mock = NewDriverRequest::mocks(1)[0].clone();
    let mock_db = Driver::insert(mock.clone(), &pool).await.unwrap();

    assert!(Driver::delete(mock_db.id, &pool).await.is_ok());

    assert!(Driver::get_by_id(mock_db.id, &pool).await.is_err());

    // Truncate db
    assert!(Driver::truncate(&pool).await.is_ok());
}
