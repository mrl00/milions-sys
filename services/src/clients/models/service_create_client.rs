use domain::models::{
    client::{ClientStatus, CreateClientModel},
    contact::CreateContactModel,
    location::CreateLocationModel,
};
use domain::types::{alphabetic::Alphabetic, doc::Doc};

#[derive(Debug)]
pub struct ServiceCreateClient {
    pub client: CreateClientModel,
    pub contact: Option<CreateContactModel>,
    pub location: Option<CreateLocationModel>,
}

#[derive(Debug)]
pub struct ServiceCreateClientModel {
    pub tx_name: Alphabetic,
    pub tx_status: ClientStatus,
    pub tx_doc: Doc,
}
