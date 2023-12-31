use crate::serializer::{FileOperatedOn, Operation};
use crate::view::{SqliteView, View};
use async_trait::async_trait;
use colored::Colorize;
use rusqlite::params;
use std::error::Error;
use tokio_rusqlite::Connection;

const OPERATIONS_SCHEMA: &str = r#"
create table IF NOT EXISTS operations
                (
                    user          TEXT not null,
                    users_group   TEXT not null,
                    executable    TEXT not null,
                    syscall       TEXT not null,
                    operation_key TEXT not null,
                    unix_observation_time INTEGER
                );
"#;
const FILES_SCHEMA: &str = r#"
create table IF NOT EXISTS operated_on_files
                (
                    absolute_path TEXT not null,
                    unix_observation_time INTEGER
                );
"#;
const INSERT_OPERATION: &'static str = "INSERT INTO operations (user,users_group,executable,syscall,operation_key,unix_observation_time) VALUES (?1,?2,?3,?4,?5,?6)";

const INSERT_FILE: &'static str =
    "INSERT INTO operated_on_files (absolute_path, unix_observation_time) VALUES (?1,?2)";
impl SqliteView {
    pub(crate) fn new(db_path: &str) -> Self {
        log::debug!("Script ran: {}", OPERATIONS_SCHEMA);
        Self::create_schema_if_not_present(db_path)
            .expect("Fatal: could not initiate schema, check if your chosen database exists.");
        return Self {
            db_path: db_path.to_string(),
        };
    }

    fn create_schema_if_not_present(db_path: &str) -> Result<(), Box<dyn Error>> {
        log::info!("Opening an {} connection", "Sqlite".yellow());
        let conn = rusqlite::Connection::open(db_path)?;
        log::debug!("Injecting a {} to {}", "schema".green(), db_path.green());
        let _ = conn.execute(OPERATIONS_SCHEMA, [])?;
        let _ = conn.execute(FILES_SCHEMA, [])?;
        return Ok(());
    }
}

#[async_trait]
impl View for SqliteView {
    async fn update(&self, operation: Operation) -> Result<(), ()> {
        log::info!("Opening an {} connection", "Sqlite".yellow());
        let db_connection = Connection::open(self.db_path.clone()).await.unwrap();
        log::info!(
            "Inserting {}",
            serde_json::to_string(&operation).unwrap().green()
        );
        db_connection
            .call(move |conn| {
                conn.execute(
                    INSERT_OPERATION,
                    params![
                        operation.user,
                        operation.group,
                        operation.executable,
                        operation.syscall,
                        operation.key.to_string(),
                        operation.timestamp
                    ],
                )
            })
            .await
            .expect("Failed to insert operation data to the database");
        Ok(())
    }

    async fn report(&self, files: FileOperatedOn) -> Result<(), ()> {
        log::info!("Opening an {} connection", "Sqlite".yellow());
        let db_connection = Connection::open(self.db_path.clone()).await.unwrap();

        db_connection
            .clone()
            .call(move |conn| {
                conn.execute(
                    INSERT_FILE,
                    params![files.name.clone(), files.timestamp.clone()],
                )
            })
            .await
            .expect("Failed to insert operation data to the database");
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::serializer::{FileOperatedOn, Operation, OperationKey};
    use crate::view::{SqliteView, View};
    use futures::executor;
    use tempfile::tempdir;

    const DB_FILE_NAME: &str = "test.sqlite";
    const COMPLIANT_LOG_LINE: &str = "type=SYSCALL msg=audit(1698576562.955:570): arch=c000003e syscall=257 success=yes exit=3 a0=ffffff9c a1=55a917750550 a2=90800 a3=0 items=1 ppid=20120 pid=20680 auid=1000 uid=1000 gid=1000 euid=1000 suid=1000 fsuid=1000 egid=1000 sgid=1000 fsgid=1000 tty=pts2 ses=14 comm=\"ls\" exe=\"/usr/bin/ls\" subj=unconfined key=\"READ\"ARCH=x86_64 AUID=\"maciek\" UID=\"maciek\" GID=\"maciek\" EUID=\"maciek\" SUID=\"maciek\" FSUID=\"maciek\" EGID=\"maciek\" SGID=\"maciek\"";
    const FILE_LOG_LINE: &str = "type=PATH msg=audit(1364481363.243:24287): item=0 name=\"/etc/ssh/sshd_config\" inode=409248 dev=fd:00 mode=0100600 ouid=0 ogid=0 rdev=00:00 obj=system_u:object_r:etc_t:s0  objtype=NORMAL cap_fp=none cap_fi=none cap_fe=0 cap_fver=0";
    #[test]
    fn if_file_has_been_operated_on_check_persistence() {
        let temporary_sqlite_directory = tempdir().unwrap();
        let db_path = temporary_sqlite_directory.path().join(DB_FILE_NAME);
        let sqlite_view = SqliteView::new(db_path.to_str().unwrap());
        insert_test_values(sqlite_view);
        assert_one_entry_is_present_and_has_values_the_same_as_parsed_operation(
            db_path.clone().into_os_string().into_string().unwrap(),
        );
        assert_one_entry_is_present_and_has_values_the_same_as_parsed_file_operated_on(
            db_path.into_os_string().into_string().unwrap(),
        );
    }

    fn insert_test_values(sqlite_view: SqliteView) {
        executor::block_on(
            sqlite_view.update(Operation::new(COMPLIANT_LOG_LINE.to_string()).unwrap()),
        )
        .unwrap();
        executor::block_on(
            sqlite_view.report(
                FileOperatedOn::new(FILE_LOG_LINE.to_string(), 1701533809.to_string())
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .clone(),
            ),
        ).unwrap();
    }

    fn assert_one_entry_is_present_and_has_values_the_same_as_parsed_operation(db_path: String) {
        let result = get_last_entry_from_db(db_path).unwrap();
        let expected = Operation::new(COMPLIANT_LOG_LINE.to_string()).unwrap();

        assert_eq!(expected.key, result.key);
        assert_eq!(expected.syscall, result.syscall);
        assert_eq!(expected.executable, result.executable);
        assert_eq!(expected.user, result.user);
        assert_eq!(expected.group, result.group);
    }

    fn assert_one_entry_is_present_and_has_values_the_same_as_parsed_file_operated_on(
        db_path: String,
    ) {
        let result = get_last_entry_operated_on_from_db(db_path).unwrap();
        let expected = FileOperatedOn::new(FILE_LOG_LINE.to_string(), "123".to_string()).unwrap();
        let entry = expected.get(0);
        assert_eq!(entry.unwrap().name, result.name);
        //assert_eq!(expected.timestamp, result.timestamp);
    }

    fn get_last_entry_from_db(db_path: String) -> Result<Operation, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(db_path).unwrap();
        let mut stmt = conn.prepare(
            "SELECT user, users_group, executable, syscall, operation_key, unix_observation_time FROM operations",
        )?;
        let operations_iter = stmt.query_map([], |row| {
            Ok(Operation {
                user: row.get(0)?,
                group: row.get(1)?,
                executable: row.get(2)?,
                syscall: row.get(3)?,
                timestamp: row.get(4)?,
                key: OperationKey::READ,
            })
        })?;
        let result = operations_iter.last().unwrap()?;
        return Ok(result);
    }
    fn get_last_entry_operated_on_from_db(
        db_path: String,
    ) -> Result<FileOperatedOn, Box<dyn std::error::Error>> {
        let conn = rusqlite::Connection::open(db_path).unwrap();
        let mut stmt =
            conn.prepare("SELECT unix_observation_time, absolute_path FROM operated_on_files")?;
        let operations_iter = stmt.query_map([], |row| {
            Ok(FileOperatedOn {
                name: row.get(1)?,
                timestamp: 1701533809.to_string()
            })
        })?;
        let result = operations_iter.last().unwrap()?;
        return Ok(result);
    }
}
