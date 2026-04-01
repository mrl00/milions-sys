use uuid::Uuid;

use crate::{
    errors::ClientError,
    models::db::client_row::ClientStatus,
    types::{alphabetic::Alphabetic, doc::Doc},
};

#[derive(Debug)]
pub struct ClientEntity {
    id: Uuid,
    name: Alphabetic,
    status: ClientStatus,
    doc: Doc,
}

impl ClientEntity {
    pub fn new(id: Uuid, name: Alphabetic, doc: Doc) -> Self {
        Self {
            id,
            name,
            status: ClientStatus::Active,
            doc,
        }
    }

    pub fn reconstitute(id: Uuid, name: Alphabetic, status: ClientStatus, doc: Doc) -> Self {
        Self {
            id,
            name,
            status,
            doc,
        }
    }

    // --- accessors ---

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &Alphabetic {
        &self.name
    }

    pub fn status(&self) -> &ClientStatus {
        &self.status
    }

    pub fn doc(&self) -> &Doc {
        &self.doc
    }

    // --- domain operations ---

    pub fn change_name(&mut self, name: Alphabetic) {
        self.name = name;
    }

    pub fn activate(&mut self) -> Result<(), ClientError> {
        if self.status == ClientStatus::Active {
            return Err(ClientError::AlreadyActive { uuid: self.id });
        }
        self.status = ClientStatus::Active;
        Ok(())
    }

    pub fn deactivate(&mut self) -> Result<(), ClientError> {
        if self.status == ClientStatus::Inactive {
            return Err(ClientError::AlreadyInactive { uuid: self.id });
        }
        self.status = ClientStatus::Inactive;
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.status == ClientStatus::Active
    }
}
