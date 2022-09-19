pub mod player;

use crate::StoreConnection;
use diesel::connection::SimpleConnection;

pub fn health_check(conn: &mut StoreConnection) -> bool {
    let result = conn.batch_execute("SELECT 1");
    result.is_ok()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    #[serial_test::serial]
    fn test_health_check() {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("mysql://root:@127.0.0.1:4000/test?charset=utf8mb4".to_owned());
        let mut conn = establish_connection(database_url).unwrap();
        let ok = health_check(&mut conn);
        assert!(ok)
    }
}
