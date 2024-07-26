use eyre::eyre;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Arguments, Encode, Sqlite, Type};

#[derive(Clone, Default)]
pub struct QueryArguments<'a>(SqliteArguments<'a>);

impl<'a> QueryArguments<'a> {
    pub fn add<T>(&mut self, arg: T) -> eyre::Result<()>
    where
        T: Encode<'a, Sqlite> + Type<Sqlite> + 'a,
    {
        self.0.add(arg).map_err(|e| eyre!(e))
    }

    pub fn into_inner(self) -> SqliteArguments<'a> {
        self.0
    }
}
