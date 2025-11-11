use sea_orm::entity::prelude::*;

#[derive(EnumIter, DeriveActiveEnum, Debug, PartialEq, Eq, Clone)]
#[sea_orm(rs_type = "String", db_type = "Text")]
pub enum EventType {
    #[sea_orm(string_value = "exec_start")]
    ExecStart,
    #[sea_orm(string_value = "exec_stop")]
    ExecStop,
    #[sea_orm(string_value = "before_sleep")]
    BeforeSleep,
    #[sea_orm(string_value = "after_sleep")]
    AfterSleep,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "data")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub event_type: EventType,
    pub timestamp: DateTimeUtc,
}

impl ActiveModelBehavior for ActiveModel {}
