use anyhow;
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

pub struct Meal {
    pub id: i64,
    pub meal: String,
    pub day: chrono::NaiveDate,
    pub price: i64,
    pub students: String,
}
#[derive(Serialize, Deserialize)]
pub struct SerialMeal {
    pub id: i64,
    pub meal: String,
    pub day: String,
    pub price: i64,
    pub students: String,
}
impl SerialMeal {
    pub fn new(meal: &Meal) -> SerialMeal {
        SerialMeal {
            id: meal.id,
            meal: meal.meal.clone(),
            day: meal.day.to_string(),
            price: meal.price,
            students: meal.students.clone(),
        }
    }
}
pub struct Student {
    pub id: i64,
    pub name: String,
}
//imlement error, debug, and into response. then replace it in main.
//#[derive(Debug, Error)]
pub enum ResponseError {
    //#[error("Bad Request")]
    BadRequest,
    //#[error("User Not Found")]
    MealNotFound,
    //#[error("Server Error")]
    InternalServerError,
}
impl IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::MealNotFound => (StatusCode::NOT_FOUND, "Meal Not Found"),
        };
        (status, Json(json!({"error":error_message}))).into_response()
    }
}
