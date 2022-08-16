use sea_orm::entity::prelude::*;
use sea_orm::{Set, QueryOrder};
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use chrono::{NaiveDateTime, DateTime, Utc}; // TODO: Replace by time-rs as soon sea-orm support it
use crate::database::orm::get_conn;
use sea_orm::ActiveValue::NotSet;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub uuid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub async fn list() -> anyhow::Result<Vec<Model>> {
    let conn = get_conn().await;

    Entity::find()
        .order_by_asc(Column::Email)
        .all(conn)
        .await
        .map_err(|e| {
            println!("{}", e);
            anyhow::anyhow!(e)
        })
        
}

pub async fn create(first_name: &str, last_name: &str, email: &str, password: &str) -> anyhow::Result<()> {
    let conn = get_conn().await;
    let model = ActiveModel {
        first_name: Set(first_name.into()),
        last_name: Set(last_name.into()),
        email: Set(email.into()),
        password: Set(password.into()),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    model.insert(conn).await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}

pub async fn update(uuid: Uuid, first_name: &str, last_name: &str, email: &str, password: &str) -> anyhow::Result<()> {
    let conn = get_conn().await;

    let model = find_by_uuid(uuid).await?;

    let mut user: ActiveModel = model.unwrap().into();

    user.first_name = Set(first_name.into());
    user.last_name = Set(last_name.into());
    user.email = Set(email.into());
    user.password = Set(password.into());
    user.updated_at = Set(Utc::now().naive_utc());

    user.update(conn).await.map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}

pub async fn delete(uuid: Uuid) -> anyhow::Result<()> {
    let conn = get_conn().await;

    let model = find_by_uuid(uuid).await?;

    model.unwrap().delete(conn).await.map_err(|e| anyhow::anyhow!(e))?;
    // let mut user: ActiveModel = model.unwrap().into();

    // user.update(conn).await.map_err(|e| anyhow::anyhow!(e))?;

    Ok(())
}

pub async fn find_by_email(email: &str) -> anyhow::Result<Option<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Email.eq(email))
        .one(conn)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn find_by_uuid(uuid: Uuid) -> anyhow::Result<Option<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Uuid.eq(uuid))
        .one(conn)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        dotenv::dotenv().ok();
        create("First Name", "Last Name", "test@example.com", "123456").await.expect("create user failed");
    }

    #[tokio::test]
    async fn test_find_user() {
        dotenv::dotenv().ok();
        let user = find_by_email("test@example.com")
            .await
            .expect("find user failed")
            .expect("user not found");
        assert_eq!(user.password, "123456");
    }
}