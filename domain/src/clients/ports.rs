use crate::{
    errors::ClientError,
    models::client::{ClientModel, CreateClientModel, UpdateClientModel},
};

pub trait ClientQueryRepository: Send + Sync {
    fn find_by_uuid(
        &self,
        uuid: uuid::Uuid,
    ) -> impl Future<Output = Result<Option<ClientModel>, ClientError>>;
    fn find_by_document(
        &self,
        document: String,
    ) -> impl Future<Output = Result<Option<ClientModel>, ClientError>>;
    fn find_all(&self) -> impl Future<Output = Result<Vec<ClientModel>, ClientError>>;
}

pub trait ClientMutationRepository: Send + Sync {
    fn create(
        &self,
        c: CreateClientModel,
    ) -> impl Future<Output = Result<ClientModel, ClientError>>;

    fn update(
        &self,
        uuid: uuid::Uuid,
        u: UpdateClientModel,
    ) -> impl Future<Output = Result<ClientModel, ClientError>>;

    fn activate(&self, uuid: uuid::Uuid) -> impl Future<Output = Result<ClientModel, ClientError>>;

    fn deactivate(
        &self,
        uuid: uuid::Uuid,
    ) -> impl Future<Output = Result<ClientModel, ClientError>>;
}
