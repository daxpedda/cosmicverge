use chrono::{DateTime, Utc};
use cosmicverge_shared::permissions::{Permission, Service};

pub struct PermissionGroup {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

pub struct Statement {
    pub id: i32,
    pub service: String,
    pub permission: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl PermissionGroup {
    pub async fn create<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        name: String,
        executor: E,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            "INSERT INTO permission_groups (name) VALUES ($1) RETURNING id, name, created_at",
            name
        )
        .fetch_one(executor)
        .await
    }

    pub async fn add_permission<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        permission: Permission,
        executor: E,
    ) -> Result<Statement, sqlx::Error> {
        sqlx::query_as!(
            Statement,
            "INSERT INTO permission_group_statements (permission_group_id, service, permission) VALUES ($1, $2, $3) RETURNING id, service, permission, created_at",
            self.id, permission.service().to_string(), permission.to_string()
        )
        .fetch_one(executor)
        .await
    }

    pub async fn add_all_service_permissions<
        'e,
        A: sqlx::Acquire<'e, Database = sqlx::Postgres> + Send,
    >(
        &self,
        service: Service,
        executor: A,
    ) -> Result<Statement, sqlx::Error> {
        let mut conn = executor.acquire().await?;
        sqlx::query!(
            "DELETE FROM permission_group_statements WHERE permission_group_id = $1 AND service = $2",
            self.id, service.to_string()
        )
        .execute(&mut *conn)
        .await?;
        sqlx::query_as!(
            Statement,
            "INSERT INTO permission_group_statements (permission_group_id, service) VALUES ($1, $2) RETURNING id, service, permission, created_at",
            self.id, service.to_string()
        )
        .fetch_one(&mut *conn)
        .await
    }

    pub async fn remove_permission<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        permission: Permission,
        executor: E,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM permission_group_statements WHERE permission_group_id = $1 AND permission = $2",
            self.id, permission.to_string()
        )
        .execute(executor)
        .await.map(|_| ())
    }

    pub async fn remove_all_service_permissions<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        &self,
        service: Service,
        executor: E,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
                "DELETE FROM permission_group_statements WHERE permission_group_id = $1 AND service = $2",
                self.id, service.to_string()
            )
            .execute(executor)
            .await.map(|_| ())
    }
}

impl Statement {
    pub async fn delete<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM permission_group_statements WHERE id = $1",
            self.id
        )
        .execute(executor)
        .await
        .map(|_| ())
    }
}
