pub struct Pool(
    deadpool_postgres::Pool
); impl Pool {
    pub async fn get(&self) {
        let w = self.0.get().await;
    }
}
