use std::fmt::Write;

pub struct SelectStatement<'a> {
    table: &'a str,
    columns: &'a [&'a str],
    joins: &'a [Join<'a>],
    condition: Option<Condition<'a>>,
    group_by: Option<&'a str>,
    order_by: Option<&'a [&'a str]>,
    limit: Option<u32>,
}

pub fn select(table: &str) -> SelectStatement {
    SelectStatement {
        table,
        columns: &[],
        joins: &[],
        condition: None,
        group_by: None,
        order_by: None,
        limit: None,
    }
}

impl<'a> SelectStatement<'a> {
    pub fn columns(mut self, columns: &'a [&'a str]) -> Self {
        self.columns = columns;
        self
    }

    pub fn joins(mut self, joins: &'a [Join<'a>]) -> Self {
        self.joins = joins;
        self
    }

    pub fn condition(mut self, condition: impl Into<Condition<'a>>) -> Self {
        self.condition = Some(condition.into());
        self
    }

    pub fn group_by(mut self, group_by: &'a str) -> Self {
        self.group_by = Some(group_by);
        self
    }

    pub fn order_by(mut self, order_by: &'a [&'a str]) -> Self {
        self.order_by = Some(order_by);
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn to_sql(&self) -> String {
        let mut sql = String::new();

        sql.push_str("SELECT ");
        sql.push_list(self.columns, |sql, col| sql.push_str(col));
        sql.push_str(" FROM ");
        sql.push_str(self.table);

        for join in self.joins {
            sql.push(' ');

            let name = match join.join_type {
                JoinType::Inner => "JOIN",
                JoinType::Left => "LEFT JOIN",
            };

            sql.push_str(name);
            sql.push(' ');
            sql.push_str(join.table);

            if let Some(on) = join.on {
                sql.push_str(" ON ");
                sql.push_str(on);
            }
        }

        fn write_cond(sql: &mut String, cond: &Condition<'_>) {
            match cond {
                Condition::Atom(str) => sql.push_str(str),
            }
        }

        if let Some(condition) = &self.condition {
            sql.push_str(" WHERE ");
            write_cond(&mut sql, condition);
        }

        if let Some(group_by) = self.group_by {
            sql.push_str(" GROUP BY ");
            sql.push_str(group_by);
        }

        if let Some(order_by) = self.order_by {
            sql.push_str(" ORDER BY ");
            sql.push_list(order_by, |sql, col| sql.push_str(col));
        }

        if let Some(limit) = self.limit {
            write!(sql, " LIMIT {}", limit).unwrap();
        }

        sql
    }
}

pub enum Condition<'a> {
    Atom(&'a str),
}

impl<'a> From<&'a str> for Condition<'a> {
    fn from(str: &'a str) -> Self {
        Condition::Atom(str)
    }
}

#[derive(Clone, Copy)]
pub enum JoinType {
    Inner,
    Left,
}

#[derive(Clone)]
pub struct Join<'a> {
    join_type: JoinType,
    table: &'a str,
    on: Option<&'a str>,
}

impl<'a> Join<'a> {
    pub const fn new(join_type: JoinType, table: &'a str) -> Self {
        Join {
            join_type,
            table,
            on: None,
        }
    }

    pub const fn inner(table: &'a str) -> Self {
        Join::new(JoinType::Inner, table)
    }

    pub const fn left(table: &'a str) -> Self {
        Join::new(JoinType::Left, table)
    }

    pub const fn on(mut self, on: &'a str) -> Self {
        self.on = Some(on);
        self
    }
}

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

pub struct Placeholders(pub usize);

impl std::fmt::Display for Placeholders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 > 0 {
            f.write_char('?')?;

            for _ in 1..self.0 {
                f.write_str(",?")?;
            }
        }

        Ok(())
    }
}
