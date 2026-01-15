use crate::data::DataSource;

pub fn get_user_full_name(ds: &dyn DataSource, user_id: u64) -> Option<String> {
    ds.find_user_by_id(user_id)
        .map(|user| format!("{} {}", user.first_name, user.last_name))
}
