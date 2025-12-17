use sqlx::types::time::OffsetDateTime;
use axum::{extract::State, response::Json};
use serde::Serialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;
use crate::sec::{AppError, Claims};

#[derive(Serialize)]
pub struct UsageStatsResponse {
    pub total_accesses: i64,
    pub features_accessed: Vec<FeatureStats>,
    pub last_activity: Option<OffsetDateTime>,
}

#[derive(Serialize)]
pub struct FeatureStats {
    pub feature_name: String,
    pub access_count: i64,
    pub last_accessed: OffsetDateTime,
}

pub async fn get_my_usage_stats_handler(
    State(app_state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<Json<UsageStatsResponse>, AppError> {

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::InvalidToken)?;

    let features = sqlx::query!(
        r#"
        SELECT 
            feature_name,
            COUNT(id) as access_count,
            MAX(accessed_at) as last_accessed
        FROM usage_metrics
        WHERE user_id = $1
        GROUP BY feature_name
        ORDER BY access_count DESC
        "#,
        user_id
    )
    .fetch_all(&app_state.pool)
    .await?;

    let feature_stats: Vec<FeatureStats> = features.into_iter()
        .filter_map(|r| {
            r.last_accessed.map(|la| FeatureStats {
                feature_name: r.feature_name,
                access_count: r.access_count.unwrap_or(0),
                last_accessed: la,
            })
        })
        .collect();

    let total = sqlx::query!(
        r#"
        SELECT 
            COUNT(*) as total_count,
            MAX(accessed_at) as last_activity
        FROM usage_metrics
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(&app_state.pool)
    .await?;
    
    crate::sec::log_security_event("STATS_FETCH", Some(&claims.username), None, "Success");

    Ok(Json(UsageStatsResponse {
        total_accesses: total.total_count.unwrap_or(0),
        features_accessed: feature_stats,
        last_activity: total.last_activity,
    }))
}