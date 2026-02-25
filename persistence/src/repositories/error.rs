use sqlx::Error;

#[derive(Debug)]
pub enum PersistenceError {
    NotFound,
    Conflict,
    DatabaseError,
}

fn map_error(err: sqlx::Error) -> PersistenceError {
    match err {
        Error::RowNotFound => PersistenceError::NotFound,

        Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            PersistenceError::Conflict
        }

        _ => PersistenceError::DatabaseError,
    }
}

impl From<sqlx::Error> for PersistenceError {
    fn from(err: sqlx::Error) -> Self {
        map_error(err)
    }
}
