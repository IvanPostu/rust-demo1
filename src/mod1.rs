pub fn get_num_5() -> i32 {
    tracing::trace!("calling mod1: get_5");
    tracing::debug!("calling mod1: get_5");
    tracing::info!("calling mod1: get_5");
    tracing::warn!("calling mod1: get_5");
    tracing::error!("calling mod1: get_5");
    get_5()
}

fn get_5() -> i32 {
    5
}
