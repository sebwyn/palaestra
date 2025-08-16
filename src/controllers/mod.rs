use std::error::Error;

use serde::Serialize;

pub mod notes_controller;


#[derive(Serialize)]
pub struct Response<T: Serialize> {
    body: Option<T>,
    status: String,
    message: String
}

impl<T: Serialize> Response<T> {
    fn with_body(body: T) -> Self {
        Self { 
            status: "Ok".to_string(),
            message: "Success".to_string(),
            body: Some(body)
        }
    }

    fn with_error(error: Box<dyn Error>) -> Self {
        Self {
            status: "Error".to_string(),
            message: error.to_string(),
            body: None
        }
    }

    fn from_result(result: Result<T, Box<dyn Error>>) -> Self {
        match result {
            Ok(body) => Self::with_body(body),
            Err(e) => Self::with_error(e),
        }
    }
}