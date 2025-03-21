mod models;
mod serialize_date;
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Form, Json, Router,
};
use models::*;
//use axum_error::Error;
//use axum_error::Result;
use anyhow::Context;
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::SqlitePool,
    types::{chrono, time},
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //Connect to database
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .route("/create", get(create))
        .route("/delete/:id", get(delete_meal))
        .route("/update", get(add_student))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum_server::Server::bind(address)
        .serve(app.into_make_service())
        .await?)
}
#[debug_handler]
// Implement your own Error that implements IntoResponse
async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<SerialMeal>>, ResponseError> {
    let meals = sqlx::query_as!(Meal, "SELECT * FROM menu ORDER BY day")
        .fetch_all(&pool)
        .await
        .map_err(|_| ResponseError::MealNotFound)?;
    let serial_meals = meals.iter().map(|meal| SerialMeal::new(meal)).collect();
    Ok(Json(serial_meals))
}

async fn create(State(pool): State<SqlitePool>) -> Result<String, ResponseError> {
    let meal = String::from("Nikola Jokic");
    let day = chrono::NaiveDate::default();
    let price: i64 = 55;
    let students = String::new();
    sqlx::query!(
        "INSERT INTO menu (meal, day,price,students) VALUES (?,?,?,?)",
        meal,
        day,
        price,
        students
    )
    .execute(&pool)
    .await
    .map_err(|_| ResponseError::InternalServerError)?;
    println!("added meal");
    Ok(format!("Successfully added meal"))
}
#[debug_handler]
async fn add_student(
    State(pool): State<SqlitePool>,
    Form(student): Form<Student>,
) -> Result<String, ResponseError> {
    sqlx::query!(
        "UPDATE menu SET students = ? WHERE id = ?",
        student.name,
        student.id
    )
    .execute(&pool)
    .await
    .map_err(|_| ResponseError::InternalServerError)?;
    Ok(format!("Successfully added student"))
}
async fn delete_meal(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<String, ResponseError> {
    sqlx::query!("DELETE FROM menu WHERE id = ?", id)
        .execute(&pool)
        .await
        .map_err(|_| ResponseError::InternalServerError)?;
    Ok(format!("Successully deleted meal"))
}
