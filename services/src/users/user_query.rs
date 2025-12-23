use crate::users::models::user::User;

pub struct UserQuery;
impl UserQuery {
    pub async fn get_by_uuid(
        pool: &sqlx::PgPool,
        uuid: uuid::Uuid,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users.tb_user
            WHERE pk_user = $1
            "#,
            &uuid,
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_all(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users.tb_user
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}
