use actix_web::error;

extern crate custom_error;
use custom_error::custom_error;

custom_error! { pub MyError
    General{message:String }  = "Error: {message}",
    NotFound{message:String} = "File not found: {message}",
    Internal{message:String} = "Internal error: {message}",
    BadRequest{message:String} = "Bad request: {message}",
    Duplicated{message:String} = "Duplicated: {message}",
}

impl From<diesel::result::Error> for MyError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UniqueViolation,
                _,
            ) => {
                return MyError::Duplicated {
                    message: err.to_string(),
                };
            }
            diesel::result::Error::NotFound => {
                return MyError::NotFound {
                    message: err.to_string(),
                };
            }
            _ => {
                return MyError::General {
                    message: err.to_string(),
                };
            }
        }
    }
}

impl From<r2d2::Error> for MyError {
    fn from(err: r2d2::Error) -> Self {
        MyError::General {
            message: format!("r2d2 error: {}", err),
        }
    }
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}