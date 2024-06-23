use std::borrow::Cow;

use derive_more::From;
use lib_auth::pwd;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use sqlx::error::DatabaseError;

use super::store::{self, dbx};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
    UserTypesNotSet,
    EmailAddressAlreadyExists {
        email: String,
    },
    IdNumberAlreadyExists {
        id_number: i32,
    },
    ListLimitOverMax {
        max: i64,
        actual: i64,
    },
    BookAccessionNoAlreadyExist {
        accession_no: i32,
    },
    AccessionNoAlreadyExist {
        accession_no: i32,
    },
    PublisherAlreadyExists {
        publisher_name: String,
    },
    AuthorNameAlreadyExists {
        author_name: String,
    },
    InventoryBookIdAlreadyExists {
        book_id: i64,
    },
    FailToCreateModelManagerProvider(String),

    FailToInsertData(String),
    FailToGetData(String),
    FailToGetSingleData(String),
    FailToUpdateData(String),
    FailToDeleteData(String),

    EntityNotFound {
        entity: &'static str,
        id: i64,
    },
    UniqueViolation {
        table: String,
        constraint: String,
    },
    CannotSelectJoinTables {
        entity: &'static str,
        id: i64,
    },

    #[from]
    Pwd(pwd::Error),
    #[from]
    Dbx(dbx::Error),

    #[from]
    SerdeSerializationError(String),

    // --- External errors
    #[from]
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),

    #[from]
    SeaQuery(#[serde_as(as = "DisplayFromStr")] sea_query::error::Error),

    #[from]
    ModqlIntoSea(#[serde_as(as = "DisplayFromStr")] modql::filter::IntoSeaError),
}

impl Error {
    pub fn resolve_unique_violation<F>(self, resolver: Option<F>) -> Self
    where
        F: FnOnce(&str, &str) -> Option<Self>,
    {
        match self
            .as_database_error()
            .map(|db_error| (db_error.code(), db_error.table(), db_error.constraint()))
        {
            Some((Some(Cow::Borrowed("233505")), Some(table), Some(constraint))) => resolver
                .and_then(|fun| fun(table, constraint))
                .unwrap_or_else(|| Error::UniqueViolation {
                    table: table.to_string(),
                    constraint: constraint.to_string(),
                }),
            _ => self,
        }
    }

    pub fn as_database_error(&self) -> Option<&(dyn DatabaseError + 'static)> {
        match self {
            Error::Dbx(dbx::Error::Sqlx(sqlx_error)) => sqlx_error.as_database_error(),
            _ => None,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
