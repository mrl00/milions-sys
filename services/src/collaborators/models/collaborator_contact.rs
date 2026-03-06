use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct CollaboratorContact {
    pub pk_collaborator_contact: Uuid,
    pub idx_collaborator_contact: i64,
    pub fk_collaborator: Uuid,
    pub fk_contact: Uuid,
    pub ts_collaborator_contact_created_at: NaiveDateTime,
    pub ts_collaborator_contact_updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateCollaboratorContact {
    pub fk_collaborator: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct UpdateCollaboratorContact {
    pub pk_collaborator_contact: Uuid,
    pub fk_collaborator: Option<Uuid>,
    pub fk_contact: Option<Uuid>,
    pub ts_collaborator_contact_created_at: Option<NaiveDateTime>,
    pub ts_collaborator_contact_updated_at: Option<NaiveDateTime>,
}
