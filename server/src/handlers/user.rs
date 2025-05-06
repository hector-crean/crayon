use crate::{database::Database, AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::{DateTime, Utc};
use crayon_core::user::{ClerkJsUser, User};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;
const USERS_TABLE: &str = "user";

type DatabaseResult<T, S> = Result<Json<T>, <<S as AppState>::D as Database>::Error>;

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct CreateUserRequest {
    user: ClerkJsUser,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUserResponse {
    pub table_id: String,
    pub record_id: String,
}

pub async fn create_user<S: AppState>(
    State(state): State<S>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, <<S as AppState>::D as Database>::Error> {
    let user: User = payload.user.into();
    let user_id = user.user_id.clone();

    let (table_id, record_id) = state
        .database()
        .create(USERS_TABLE.into(), Some(user_id.into()), user)
        .await?;
    Ok(Json(CreateUserResponse {
        table_id: table_id.into(),
        record_id: record_id.into(),
    }))
}

pub async fn upsert_user<S: AppState>(
    State(state): State<S>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, <<S as AppState>::D as Database>::Error> {
    let user: User = payload.user.into();
    let table_id = USERS_TABLE;

    let record_id = Uuid::new_v4().to_string();
    // Call upsert with default ById condition
    let _updated_user = state
        .database()
        .upsert(
            (table_id.into(), record_id.clone().into()),
            user,
            Some(crate::database::UpsertCondition::ByFields(vec![
                "user_id".to_string()
            ])),
        )
        .await?;

    // Return the table_id and record_id that we used
    Ok(Json(CreateUserResponse {
        table_id: table_id.into(),
        record_id: record_id.into(),
    }))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetUserResponse {
    pub user: Option<User>,
}

pub async fn get_user<S: AppState>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<GetUserResponse>, <<S as AppState>::D as Database>::Error> {
    let user = state
        .database()
        .get::<User>((USERS_TABLE.into(), id.into()))
        .await?;
    Ok(Json(GetUserResponse { user }))
}

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct GetUsersRequest {
    pub user_ids: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, TS)]
#[ts(export)]
pub struct GetUsersResponse {
    pub users: Vec<User>,
}

pub async fn get_users<S: AppState>(
    State(state): State<S>,
    Json(payload): Json<GetUsersRequest>,
) -> Result<Json<GetUsersResponse>, <<S as AppState>::D as Database>::Error> {
    let users = state.database().get_many::<User>(USERS_TABLE.into(), payload.user_ids.iter().map(|id| id.clone().into()).collect()).await?;
    Ok(Json(GetUsersResponse { users }))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListUsersResponse {
    pub users: Vec<User>,
}

pub async fn list_users<S: AppState>(
    State(state): State<S>,
) -> Result<Json<ListUsersResponse>, <<S as AppState>::D as Database>::Error> {
    let users = state.database().list::<User>(USERS_TABLE.into()).await?;
    Ok(Json(ListUsersResponse { users }))
}
