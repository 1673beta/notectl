use sea_orm::{DatabaseConnection, EntityTrait};

pub trait VersionedEntity {
    type Entity: EntityTrait;

    fn entity() -> Self::Entity;
    fn version() -> &'static str; // TODO: ここはいずれenumにした方が都合がいいかも
}

pub trait NoteEntity: VersionedEntity {}

pub trait UserEntity: VersionedEntity {}