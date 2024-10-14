use axum::extract::{Query, State};
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use crate::{dto::*, service};

/// Register new user.

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    info!("Register new user with request: {req:?}");
    req.validate()?;
    match service::user::register(state, req).await {
        Ok(user_id) => {
            info!("Successfully register user: {user_id}");
            let resp = RegisterResponse { id: user_id };
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully register user: {e:?}");
            Err(e)
        }
    }
}

/// Active registered user.

pub async fn active(
    State(state): State<AppState>,
    Json(req): Json<ActiveRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("Active user with token: {req:?}.");
    match service::user::active(&state, req).await {
        Ok(_) => {
            info!("User successfully activated.");
            Ok(Json(MessageResponse::new("User successfully activated.")))
        }
        Err(e) => {
            info!("The user activation operation was not successful: {e:?}");
            Err(e)
        }
    }
}

/// Login user.

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    info!("Login user with request: {req:?}.");
    match service::user::login(&state, req).await {
        Ok(resp) => {
            info!("Success login user_id");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully login user error: {e:?}.");
            Err(e)
        }
    }
}

/// Login2fa user.

pub async fn login2fa(
    State(state): State<AppState>,
    Json(req): Json<Login2faRequest>,
) -> AppResult<Json<LoginResponse>> {
    info!("Two factor login user with request: {req:?}.");
    match service::user::login2fa(&state, req).await {
        Ok(resp) => {
            info!("Success login user_id: {resp:?}.");
            Ok(Json(LoginResponse::Token(resp)))
        }
        Err(e) => {
            warn!("Unsuccessfully login user error: {e:?}.");
            Err(e)
        }
    }
}

/// Logout user.

pub async fn logout(
    State(state): State<AppState>,
    user: UserClaims,
) -> AppResult<Json<MessageResponse>> {
    info!("Logout user_id: {}", user.uid);
    match service::user::logout(&state, user.uid).await {
        Ok(_) => {
            info!("Success logout user user_id: {}", user.uid);
            Ok(Json(MessageResponse::new(
                "This user has successfully logged out.",
            )))
        }
        Err(e) => {
            warn!("unsuccessfully logout user: {e:?}");
            Err(e)
        }
    }
}

/// Forgot user password.

pub async fn forget_password(
    State(state): State<AppState>,
    Query(param): Query<ForgetPasswordQueryParam>,
) -> AppResult<Json<ForgetPasswordResponse>> {
    info!("Forget password user query parameter: {param:?}");
    match service::user::forget_password(&state, param).await {
        Ok(resp) => {
            info!("Success forget password user response.");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessful forget password user: {e:?}.");
            Err(e)
        }
    }
}

/// Reset user password.

pub async fn reset_password(
    State(state): State<AppState>,
    Json(req): Json<SetPasswordRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("Reset password user: {}.", req.user_id);
    match service::user::reset_password(&state, req).await {
        Ok(_) => {
            info!("Success set new password.");
            Ok(Json(MessageResponse::new("The password has been updated.")))
        }
        Err(e) => {
            warn!("Unsuccessful set password user: {e:?}.");
            Err(e)
        }
    }
}

/// Get user profile information.

pub async fn get_profile(
    State(state): State<AppState>,
    user: UserClaims,
) -> AppResult<Json<ProfileResponse>> {
    info!("Get profile user id: {}.", user.uid);
    match service::user::get_profile(&state, user.uid).await {
        Ok(resp) => {
            info!("Success get profile user: {}.", user.uid);
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully get profile user: {e:?}.");
            Err(e)
        }
    }
}

/// Update user profile.

pub async fn update_profile(
    State(state): State<AppState>,
    user: UserClaims,
    Json(req): Json<UpdateProfileRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("Update profile user_id: {}.", user.uid);
    match service::user::update_profile(&state, user.uid, req).await {
        Ok(_) => {
            info!("Success update profile user user_id: {}.", user.uid);
            Ok(Json(MessageResponse::new("User profile updated.")))
        }
        Err(e) => {
            info!("Unsuccessful update profile user: {e:?}");
            Err(e)
        }
    }
}

/// Get user access codes.

pub async fn get_access_codes(
    State(state): State<AppState>,
    user: UserClaims,
) -> AppResult<Json<String>> {
    info!("Get profile user id: {}.", user.uid);
    match service::user::get_access_codes(&state, user.uid).await {
        Ok(resp) => {
            info!("Success get profile user: {}.", user.uid);
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully get profile user: {e:?}.");
            Err(e)
        }
    }
}
