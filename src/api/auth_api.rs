use crate::api::db;
use crate::api::types::{AuthResponse, ErrorResponse, LoginRequest, SignupRequest};
use crate::DB_POOL;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::serde::json::Json;
use rocket::time::Duration;

fn is_valid_username(username: &str) -> bool {
    !username.is_empty()
        && username.len() <= 30
        && username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[post("/api/login", format = "json", data = "<request>")]
pub async fn login(
    cookies: &CookieJar<'_>,
    request: Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (Status, Json<ErrorResponse>)> {
    let db_pool = DB_POOL.get().unwrap();
    let username = request.username.trim();

    if !is_valid_username(username) {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Username can only contain letters, numbers, and underscores".to_string(),
            }),
        ));
    }

    let user = db::get_user_by_username(db_pool, username)
        .await
        .map_err(|err| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: format!("Database error: {}", err),
                }),
            )
        })?;

    match user {
        Some(u) => {
            cookies.add_private(
                Cookie::build(("user", u.name.clone()))
                    .http_only(true)
                    .max_age(Duration::days(30)),
            );
            Ok(Json(AuthResponse {
                success: true,
                username: Some(u.name),
                display_name: None,
            }))
        }
        None => Err((
            Status::NotFound,
            Json(ErrorResponse {
                error: "Username not found".to_string(),
            }),
        )),
    }
}

#[post("/api/signup", format = "json", data = "<request>")]
pub async fn signup(
    cookies: &CookieJar<'_>,
    request: Json<SignupRequest>,
) -> Result<Json<AuthResponse>, (Status, Json<ErrorResponse>)> {
    let db_pool = DB_POOL.get().unwrap();
    let username = request.username.trim();
    let first_name = request.first_name.trim();
    let last_name = request.last_name.trim();

    if !is_valid_username(username) {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Username can only contain letters, numbers, and underscores".to_string(),
            }),
        ));
    }

    if first_name.is_empty() || last_name.is_empty() {
        return Err((
            Status::BadRequest,
            Json(ErrorResponse {
                error: "Please fill in all fields".to_string(),
            }),
        ));
    }

    let existing = db::get_user_by_username(db_pool, username)
        .await
        .map_err(|err| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: format!("Database error: {}", err),
                }),
            )
        })?;

    if existing.is_some() {
        return Err((
            Status::Conflict,
            Json(ErrorResponse {
                error: "Username already taken".to_string(),
            }),
        ));
    }

    let user = db::create_user(db_pool, username, first_name, last_name)
        .await
        .map_err(|err| {
            (
                Status::InternalServerError,
                Json(ErrorResponse {
                    error: format!("Failed to create user: {}", err),
                }),
            )
        })?;

    let display_name = format!("{} {}", first_name, last_name);

    cookies.add_private(
        Cookie::build(("user", user.name.clone()))
            .http_only(true)
            .max_age(Duration::days(30)),
    );

    Ok(Json(AuthResponse {
        success: true,
        username: Some(user.name),
        display_name: Some(display_name),
    }))
}

#[post("/api/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> Json<AuthResponse> {
    cookies.remove_private("user");
    // Keep api_token for Spotify API access (client credentials)
    Json(AuthResponse {
        success: true,
        username: None,
        display_name: None,
    })
}
