use axum::extract::{Query, State};
use axum::Json;
use tracing::info;

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use crate::{dto::*, service};


pub async fn list(
  State(state): State<AppState>,
  user: UserClaims,
  Query(param): Query<PageQueryParam>,
) -> AppResult<Json<GetUserListResponse>> {
  info!("Get list of user by: {} parameter: {:?}.", user.uid, param);
  match service::admin::user::list(&state, &user, param).await {
    Ok(resp) => {
      info!(
        "Success get list of users by user_id: {} response: {resp:?}.",
        user.uid
      );
      Ok(Json(resp))
    }
    Err(e) => {
      info!("Unsuccessful get user list: {e:?}");
      Err(e)
    }
  }
}
