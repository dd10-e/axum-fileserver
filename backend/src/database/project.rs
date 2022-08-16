use sea_orm::entity::prelude::*;
use sea_orm::Set;
use sea_orm::{DeleteResult, entity::*, query::*, tests_cfg::cake};
use serde::{Deserialize, Serialize};
use serde_json::value::Value as JsonValue;
use uuid::Uuid;
use chrono::{NaiveDateTime, DateTime, Utc}; // TODO: Replace by time-rs as soon sea-orm support it
use cap_std::{
    ambient_authority,
    fs::{Dir, File, OpenOptions},
    path::Path,
};

use crate::database::orm::get_conn;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "projects")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: Uuid,
    pub number: i32,
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub description: String,
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
        .all(conn)
        .await
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn create(name: &str, description: &str) -> anyhow::Result<()> {
    let conn = get_conn().await;
    let active_model = ActiveModel {
        name: Set(name.into()),
        description: Set(description.into()),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };
    let model: Model = active_model.insert(conn).await.map_err(|e| anyhow::anyhow!(e))?;
    
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");
    let cwd = Dir::open_ambient_dir(root.clone(), ambient_authority()).expect("!!!");
    
    cwd.create_dir(model.uuid.to_string()).expect("Can't create folder");

    Ok(())
}

pub async fn update(project: Model, name: &str, description: &str) -> anyhow::Result<Option<Model>> {
    let conn = get_conn().await;

    let mut project: ActiveModel = project.into();

    // Update name attribute
    project.name = Set(name.into());
    project.description = Set(description.into());

    let project: Option<Model> =  project
        .update(conn)
        .await
        .map_err(|e| anyhow::anyhow!(e))
        .ok();

    Ok(project)
}

pub async fn search(query: &str) -> anyhow::Result<Vec<Model>> {
    let conn = get_conn().await;
    
    Entity::find()
        .filter(Column::Name.contains(query))
        .all(conn)
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

pub async fn delete_by_uuid(uuid: Uuid) -> anyhow::Result<()> {
    let conn = get_conn().await;

    let model = 
        find_by_uuid(uuid)
        .await?;

    model.unwrap()
        .delete(conn)
        .await?;

    Ok(())
}