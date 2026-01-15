use test_project::{
    data::{DataSource, User},
    user_service::get_user_full_name,
};

struct DataSourceMock;
impl DataSource for DataSourceMock {
    fn find_user_by_id(&self, id: u64) -> Option<User> {
        Some(User {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        })
    }
}

#[test]
fn test_get_user_full_name() {
    let result = get_user_full_name(&DataSourceMock, 1);
    assert_eq!(Some("John Doe".to_string()), result);
}
