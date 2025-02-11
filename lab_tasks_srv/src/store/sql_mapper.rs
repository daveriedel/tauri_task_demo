pub trait FromSqliteRow {
    fn from_row(row: &rusqlite::Row) -> Result<Self, rusqlite::Error>
    where
        Self: Sized;
}
