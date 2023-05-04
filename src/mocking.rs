use crate::models::{Driver, NewDriverRequest};

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

impl Driver {
    /// Generate list of randomaized mocks of NewDriverRequest NewDriverRequest
    pub fn mocks(count: i32) -> Vec<Driver> {
        use fakeit::{
            company,
            name::{first, last},
        };
        use rand::Rng;

        let mut mocks: Vec<Driver> = Vec::new();

        for _i in 0..count {
            let mut rng = rand::thread_rng();

            let id = uuid::Uuid::new_v4();
            let first_name = first();
            let last_name = last();
            let email = format!("{}.{}@{}.com", first_name, last_name, company::bs());
            let phone = format!("0-{}", rng.gen_range(0..1_000_000_000));

            mocks.push(Driver {
                id,
                first_name,
                last_name,
                email,
                phone,
            });
        }

        mocks
    }
}
