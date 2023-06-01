use diesel::r2d2::{self, ConnectionManager};
use diesel::SqliteConnection;
use std::sync::RwLock;

use crate::objects::users::Users;
pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DBConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::SqliteConnection>>;

pub struct Data {
    pub user_list: RwLock<Users>,
    pub db_pool: Pool,
    pub token_expires: chrono::Duration,
    pub fs_root: String
}

impl Data {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let fs_root = std::env::var("FILES_ROOT").unwrap_or(String::from("www"));
        let database_url = std::env::var("DATABASE_URL").unwrap_or(String::from("db/main.sqlite"));
        log::info!("App State Init: DATABASE_URL -> {database_url}");
        let token_exp_str =
            std::env::var("TOKEN_EXPIRATION").unwrap_or("30 minutes".to_owned());
        let token_expires_std = parse_duration::parse(&token_exp_str)
            .unwrap_or(std::time::Duration::from_secs(30 * 60));
        let token_expires =
            chrono::Duration::from_std(token_expires_std).unwrap_or(chrono::Duration::minutes(30));
        log::info!("App State Init: TOKEN_EXPIRATION -> {token_expires:?} ({token_exp_str})");
        Self {
            db_pool: Pool::builder()
                .build(ConnectionManager::new(database_url))
                .unwrap(),
            user_list: RwLock::new(Users::new()),
            token_expires,
            fs_root
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        let mut users = self.user_list.write().unwrap();
        let mut db_conn = self.db_pool.get().unwrap();
        users.save(&mut db_conn);
    }
}
