pub mod client_error;
pub mod contact_error;
pub mod infra_error;
pub mod location_error;
pub mod projects_error;

// enums
pub use client_error::ClientError;
pub use contact_error::ContactError;
pub use infra_error::InfraError;
pub use location_error::LocationError;

// contextualizadores de infra — compartilhados entre todos os services
pub use infra_error::{BeginTransactionSnafu, CommitTransactionSnafu, DatabaseSnafu};

// contextualizadores de client
pub use client_error::{
    AlreadyExistsSnafu as ClientAlreadyExistsSnafu,
    ContactNotFoundSnafu as ClientContactNotFoundSnafu,
    LocationNotFoundSnafu as ClientLocationNotFoundSnafu, NotFoundSnafu as ClientNotFoundSnafu,
};

// contextualizadores de location
pub use location_error::{
    AlreadyExistsSnafu as LocationAlreadyExistsSnafu, InvalidFieldSnafu,
    NotFoundSnafu as LocationNotFoundSnafu,
};

// contextualizadores de contact
pub use contact_error::{
    AlreadyExistsSnafu as ContactAlreadyExistsSnafu, NotFoundSnafu as ContactNotFoundSnafu,
    PhoneAlreadyExistsSnafu, PhoneNotFoundSnafu,
};
