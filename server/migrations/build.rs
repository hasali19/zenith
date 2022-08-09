use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

struct Migration {
    name: String,
    path: PathBuf,
    kind: MigrationKind,
}

enum MigrationKind {
    Sql,
    Rust,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=migrations");

    let mut migrations = vec![];

    for entry in std::fs::read_dir("migrations")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let mod_path = path.join("mod.rs");
            let name = path.file_name().unwrap();
            if mod_path.is_file() {
                migrations.push(Migration {
                    name: name.to_str().unwrap().to_owned(),
                    path: mod_path,
                    kind: MigrationKind::Rust,
                });
            }
        } else if matches!(path.extension(), Some(ext) if ext == "sql" || ext == "rs") {
            let ext = match path.extension() {
                Some(ext) => ext,
                None => continue,
            };

            let kind = if ext == "sql" {
                MigrationKind::Sql
            } else if ext == "rs" {
                MigrationKind::Rust
            } else {
                continue;
            };

            let name = path.file_stem().unwrap().to_str().unwrap().to_owned();

            migrations.push(Migration { name, path, kind });
        }
    }

    migrations.sort_by(|a, b| a.name.cmp(&b.name));

    let mut out = File::create("src/migrations.rs")?;

    for migration in &migrations {
        if let MigrationKind::Rust = migration.kind {
            let name = &migration.name;
            let path = migration.path.display();
            writeln!(out, "#[path = \"../{path}\"]")?;
            writeln!(out, "mod _{name};")?;
        }
    }

    writeln!(out, "use sqlx::SqliteConnection;")?;
    writeln!(out)?;

    for migration in &migrations {
        let fn_name = format!("_{}", migration.name);

        writeln!(
            out,
            "async fn {fn_name}(conn: &mut SqliteConnection) -> eyre::Result<()> {{"
        )?;

        match migration.kind {
            MigrationKind::Sql => {
                let path = migration.path.display();
                writeln!(out, "    sqlx::query(include_str!(\"../{path}\"))",)?;
                writeln!(out, "        .execute(conn)")?;
                writeln!(out, "        .await?;")?;
                writeln!(out, "    Ok(())")?;
            }
            MigrationKind::Rust => {
                writeln!(out, "    {fn_name}::execute(conn).await")?;
            }
        }

        writeln!(out, "}}")?;
        writeln!(out)?;
    }

    writeln!(
        out,
        "pub(super) fn collect(migrator: &mut super::Migrator) {{"
    )?;

    for migration in &migrations {
        let mut hasher = sha2::Sha256::new();

        if matches!(migration.path.file_name(), Some(name) if name == "mod.rs") {
            hash_dir(&mut hasher, migration.path.parent().unwrap())?;
        } else {
            hasher.update(std::fs::read(&migration.path)?);
        }

        let hash = hasher.finalize();
        let name = &migration.name;

        writeln!(out, "    migrator.push_migration(")?;
        writeln!(out, "        \"{name}\",")?;
        writeln!(out, "        Box::new(|conn| Box::pin(_{name}(conn))),")?;
        writeln!(out, "        {hash:?},")?;
        writeln!(out, "    );")?;
    }

    writeln!(out, "}}")?;

    Ok(())
}

fn hash_dir(hasher: &mut Sha256, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut paths = std::fs::read_dir(path)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;

    paths.sort();

    for path in paths {
        if path.is_file() {
            hasher.update(std::fs::read(path)?);
        } else if path.is_dir() {
            hash_dir(&mut *hasher, &path)?;
        }
    }

    Ok(())
}
