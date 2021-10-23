use std::fmt::Write;

pub struct SqlPlaceholders(pub usize);

impl std::fmt::Display for SqlPlaceholders {
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
