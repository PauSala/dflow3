use anyhow::Result;
use rusqlite::Connection;

pub struct SqliteConnection {}

impl SqliteConnection {
    pub fn new() -> Self {
        SqliteConnection {}
    }

    pub fn connection(&self) -> Result<Connection> {
        Ok(Connection::open("models.db")?)
    }

    fn table_datasources(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS datasources (
                id TEXT primary key,
                name TEXT NOT NULL,
                type TEXT NOT NULL
             )",
            (),
        )?;
        Ok(())
    }
    fn table_sql_connection(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS sql_configurations (
                datasource_id TEXT primary key,
                host TEXT NOT NULL,
                port INTEGER NOT NULL,
                user TEXT NOT NULL,
                password TEXT NOT NULL,
                db_name TEXT,
                schema TEXT,
                dialect TEXT NOT NULL,
                FOREIGN KEY(datasource_id) REFERENCES datasources(id)
            )",
            (),
        )?;
        Ok(())
    }
    fn table_models(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS models (
                id TEXT primary key,
                datasource_id TEXT NOT NULL unique,
                FOREIGN KEY(datasource_id) REFERENCES datasources(id)
             )",
            [],
        )?;
        Ok(())
    }
    fn table_tables(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS tables (
                id INTEGER NOT NULL,
                table_name TEXT NOT NULL,
                display_name TEXT NOT NULL,
                model_id TEXT NOT NULL,
                PRIMARY KEY(id, model_id)
                FOREIGN KEY(model_id) REFERENCES models(id)
             )",
            [],
        )?;
        Ok(())
    }
    fn table_columns(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS columns (
                id INTEGER NOT NULL,
                column_name TEXT NOT NULL,
                display_name TEXT NOT NULL,
                type TEXT NOT NULL,
                type_alias TEXT NOT NULL,
                table_id INTEGER NOT NULL,
                model_id TEXT NOT NULL,
                is_array BOOLEAN,
                FOREIGN KEY(table_id, model_id) REFERENCES tables(id, model_id)
             )",
            [],
        )?;
        Ok(())
    }
    fn table_relations(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS relations (
                id INTEGER PRIMARY KEY,
                pk_table INTEGER NOT NULL,
                fk_table INTEGER NOT NULL,
                pk_column INTEGER NOT NULL,
                fk_column INTEGER NOT NULL,
                model_id TEXT NOT NULL,
                CONSTRAINT pk_table_fk FOREIGN KEY (pk_table, model_id) REFERENCES tables(id, model_id),
                CONSTRAINT fk_table_fk FOREIGN KEY (fk_table, model_id) REFERENCES tables(id, model_id)
            );",
            [],
        )?;
        Ok(())
    }

    fn table_panels(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS panels (
                panel_id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                model_id TEXT NOT NULL,
                panel JSON
            );", [])?;
        Ok(())
    }

    fn table_dashboards(&self, cn: &Connection) -> Result<()> {
        cn.execute(
            "CREATE TABLE IF NOT EXISTS dashboards (
                id TEXT NOT NULL PRIMARY KEY,
                user_id TEXT NOT NULL,
                model_id TEXT NOT NULL,
                name TEXT NOT NULL,
                config JSON
            );", [])?;
        Ok(())
    }

    pub fn create_db_if_not_exists(&self) -> Result<()> {
        let cn = self.connection()?;
        self.table_datasources(&cn)?;
        self.table_sql_connection(&cn)?;
        self.table_models(&cn)?;
        self.table_tables(&cn)?;
        self.table_columns(&cn)?;
        self.table_relations(&cn)?;
        self.table_panels(&cn)?;
        self.table_dashboards(&cn)?;
        Ok(())
    }
}
