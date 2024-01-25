use crate::modules::datasource::model::configurations::sql_configuration::SqlConfig;
use anyhow::{Ok, Result};
use deadpool_postgres::Object as PgClient;
use deadpool_postgres::{Manager, ManagerConfig, Pool as PgPool, RecyclingMethod};
use deadpool_tiberius::Pool as Mssqlpool;
use rocket::tokio::sync::RwLock;
use rocket::State;
use tokio_postgres::NoTls;

pub type MssqlClient = deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager>;

pub struct SharedConnections {
    pub postgres_pool: Option<PgPool>,
    pub mssql_pool: Option<Mssqlpool>,
}

impl SharedConnections {
    async fn set_pg_pool_and_get_client(&mut self, config: &SqlConfig) -> Result<()> {
        if let None = self.postgres_pool {
            let mut pg_config = tokio_postgres::Config::new();
            pg_config.host(&config.host);
            pg_config.user(&config.user);
            pg_config.dbname(&config.db_name.as_ref().unwrap_or(&"".to_string()));
            pg_config.password(&config.password);
            let mgr_config = ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            };
            let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
            let pool = PgPool::builder(mgr).max_size(16).build()?;
            self.postgres_pool = Some(pool);
        }
        Ok(())
    }

    async fn set_mssql_pool(&mut self, config: &SqlConfig) -> Result<()> {
        if let None = self.mssql_pool {
            let pool = deadpool_tiberius::Manager::new()
                .host(&config.host)
                .port(config.port)
                .basic_authentication(&config.user, &config.password)
                .database(
                    &config
                        .db_name
                        .as_ref()
                        .expect("Database name is mandatory in mssql"),
                )
                .trust_cert()
                .max_size(30)
                .wait_timeout(1.52)
                .create_pool()?;
            self.mssql_pool = Some(pool);
        }
        Ok(())
    }

    pub async fn get_mssql_client(
        state: &State<RwLock<SharedConnections>>,
        config: &SqlConfig,
    ) -> Result<MssqlClient> {
        let read_lock = state.read().await;
        if read_lock.mssql_pool.is_none() {
            drop(read_lock);
            let mut write_lock = state.write().await;
            write_lock.set_mssql_pool(&config).await?;
            drop(write_lock);
        }
        let read_lock = state.read().await;
        let client = read_lock
            .mssql_pool
            .as_ref()
            .expect("Mssql pool should be active")
            .get()
            .await?;
        Ok(client)
    }

    pub async fn get_pg_client(
        state: &State<RwLock<SharedConnections>>,
        config: &SqlConfig,
    ) -> Result<PgClient> {
        let read_lock = state.read().await;
        if read_lock.mssql_pool.is_none() {
            drop(read_lock);
            let mut write_lock = state.write().await;
            write_lock.set_pg_pool_and_get_client(&config).await?;
            drop(write_lock);
        }
        let read_lock = state.read().await;
        let client = read_lock
            .postgres_pool
            .as_ref()
            .expect("PgPool should be active")
            .get()
            .await?;
        Ok(client)
    }
}
