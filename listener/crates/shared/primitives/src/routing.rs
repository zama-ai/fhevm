// Common routing keys for listener/consumer.
pub const FETCH_NEW_BLOCKS: &str = "fetch-new-blocks";
pub const BACKTRACK_REORG: &str = "backtrack-reorg";
pub const WATCH: &str = "control.watch";
pub const UNWATCH: &str = "control.unwatch";
pub const CLEAN_BLOCKS: &str = "clean-blocks";
pub const NEW_EVENT: &str = "new-event";
pub const CATCHUP: &str = "catchup";

pub fn consumer_new_event_routing(consumer_id: String) -> String {
    format!("{}.{}", consumer_id, NEW_EVENT)
}
