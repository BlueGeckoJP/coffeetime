use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub start_time: DateTimeUtc,
    pub end_time: Option<DateTimeUtc>, // NULL = ongoing session
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::sleep_period::Entity",
        from = "Column::Id",
        to = "super::sleep_period::Column::SessionId"
    )]
    SleepPeriods,
}

impl Related<super::sleep_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SleepPeriods.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
