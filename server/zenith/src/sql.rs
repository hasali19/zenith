pub struct InsertStatement<'a> {
    table: &'a str,
    columns: &'a [&'a str],
    values: &'a [&'a str],
    on_conflict: Option<OnConflict<'a>>,
    returning: Option<&'a [&'a str]>,
}

pub fn insert(table: &str) -> InsertStatement {
    InsertStatement {
        table,
        columns: &[],
        values: &[],
        on_conflict: None,
        returning: None,
    }
}

impl<'a> InsertStatement<'a> {
    pub fn columns(mut self, columns: &'a [&'a str]) -> Self {
        self.columns = columns;
        self
    }

    pub fn values(mut self, values: &'a [&'a str]) -> Self {
        self.values = values;
        self
    }

    pub fn on_conflict(mut self, on_conflict: OnConflict<'a>) -> Self {
        self.on_conflict = Some(on_conflict);
        self
    }

    pub fn returning(mut self, returning: &'a [&'a str]) -> Self {
        self.returning = Some(returning);
        self
    }

    pub fn to_sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("INSERT INTO ");
        sql.push_str(self.table);
        sql.push_str(" (");
        sql.push_list(self.columns, |sql, col| sql.push_str(col));
        sql.push_str(") VALUES (");
        sql.push_list(self.values, |sql, val| sql.push_str(val));
        sql.push(')');

        if let Some(on_conflict) = &self.on_conflict {
            match on_conflict {
                OnConflict::Update(update_list) => {
                    sql.push_str("ON CONFLICT DO UPDATE SET ");
                    sql.push_list(
                        update_list.columns.iter().zip(update_list.values.iter()),
                        |sql, (col, val)| {
                            sql.push_str(col);
                            sql.push_str(" = ");
                            sql.push_str(val);
                        },
                    )
                }
            }
        }

        if let Some(returning) = self.returning {
            sql.push_str(" RETURNING ");
            sql.push_list(returning, |sql, col| sql.push_str(col));
        }

        sql
    }
}

pub enum OnConflict<'a> {
    Update(UpdateList<'a>),
}

pub struct UpdateList<'a> {
    columns: &'a [&'a str],
    values: &'a [&'a str],
}

impl<'a> UpdateList<'a> {
    pub fn new() -> Self {
        UpdateList {
            columns: &[],
            values: &[],
        }
    }

    pub fn columns(mut self, columns: &'a [&'a str]) -> Self {
        self.columns = columns;
        self
    }

    pub fn values(mut self, values: &'a [&'a str]) -> Self {
        self.values = values;
        self
    }
}

trait StringExt {
    fn push_list<T>(
        &mut self,
        list: impl IntoIterator<Item = T>,
        push_item: impl Fn(&mut String, T),
    );
}

impl StringExt for String {
    fn push_list<T>(
        &mut self,
        list: impl IntoIterator<Item = T>,
        mut push_item: impl FnMut(&mut String, T),
    ) {
        let mut iter = list.into_iter();
        if let Some(value) = iter.next() {
            push_item(self, value);

            for value in iter {
                self.push_str(", ");
                push_item(self, value);
            }
        }
    }
}
