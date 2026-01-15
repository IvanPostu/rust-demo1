pub struct User {
    pub first_name: String,
    pub last_name: String,
}

pub trait DataSource {
    fn find_user_by_id(&self, id: u64) -> Option<User>;
}
