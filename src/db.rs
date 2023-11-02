use tokio::io::Result;

const DBFILE: &str = "db.json";
pub type DB = Vec<String>;

pub async fn get_todos() -> Result<DB> {
    let exists = tokio::fs::try_exists(DBFILE).await?;

    if !exists {
        let db = Vec::new();
        save_db(db).await?;
    }

    let f = tokio::fs::read(DBFILE).await?;
    let db: DB = serde_json::from_slice(&f)?;
    Ok(db)
}

pub async fn save_db(db: DB) -> Result<()> {
    let json = serde_json::to_vec(&db)?;
    tokio::fs::write(DBFILE, &json).await?;
    Ok(())
}

pub async fn add_todo(todo: String) -> Result<()> {
    let mut db = get_todos().await?;
    db.push(todo);
    save_db(db).await?;
    Ok(())
}

pub async fn delete_todo(id: usize) -> Result<()> {
    let mut db = get_todos().await?;
    db.remove(id);
    save_db(db).await?;
    Ok(())
}
