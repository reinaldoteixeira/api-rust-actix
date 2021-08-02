use rusqlite::{Connection, Error};

pub async fn connection() -> Result<Connection, Error> {
    let conn = Connection::open("src/database/sqlite.db")?;

    match conn.execute(
        "
        create table if not exists users (
            id text primary key,
            name text not null,
            age text not null
          )",
        [],
    ) {
        Ok(_) => {}
        Err(err) => {
            panic!("this is a terrible mistake! {}",  err);
        }
    }

    Ok(conn)
}