use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirming a pending subscriber", skip(parameters))]
pub async fn subscriptions_confirm(
    pool: web::Data<PgPool>,
    parameters: web::Query<Parameters>,
) -> HttpResponse {
    let id = match get_subscriber_id_from_token(&pool, &parameters.subscription_token).await {
        Ok(subscriber_id) => subscriber_id,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match id {
        Some(subscriber_id) => {
            if confirm_subscriber(&pool, subscriber_id).await.is_ok() {
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
        None => HttpResponse::BadRequest().finish(),
    }
}

#[tracing::instrument(name = "Mark subscriber", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE subscriptions
        SET status = 'confirmed'
        WHERE id = $1
        "#,
        subscriber_id
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(pool, subscription_token))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT subscriber_id FROM subscription_tokens
        WHERE subscription_token = $1
        "#,
        subscription_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query.");
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}
