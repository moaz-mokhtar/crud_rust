use crate::{
    db::DbClient,
    models::{Driver, NewDriverRequest},
};

impl NewDriverRequest {
    /// Generate list of randomaized mocks of NewDriverRequest NewDriverRequest
    pub fn mocks(count: i32) -> Vec<NewDriverRequest> {
        use fakeit::{
            company,
            name::{first, last},
        };
        use rand::Rng;

        let mut mocks: Vec<NewDriverRequest> = Vec::new();

        for _i in 0..count {
            let mut rng = rand::thread_rng();

            let first_name = first();
            let last_name = last();
            let email = format!("{}.{}@{}.com", first_name, last_name, company::bs());
            let phone = format!("0-{}", rng.gen_range(0..1_000_000_000));

            mocks.push(NewDriverRequest {
                first_name,
                last_name,
                email,
                phone,
            });
        }

        mocks
    }
}

#[actix_web::test]
async fn test_mocks() {
    let mocks = NewDriverRequest::mocks(10);
    dbg!(&mocks);
    assert_eq!(10, mocks.len());
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
    assert_eq!(inserted_driver.first_name, mock.first_name);
    assert_eq!(inserted_driver.last_name, mock.last_name);
    assert_eq!(inserted_driver.email, mock.email);
    assert_eq!(inserted_driver.phone, mock.phone);

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
