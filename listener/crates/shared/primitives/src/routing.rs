// Common routing keys for listener/consumer.
pub const FETCH_NEW_BLOCKS: &str = "fetch-new-blocks";
pub const FETCH_FINAL_BLOCK: &str = "fetch-final-block";
pub const BACKTRACK_REORG: &str = "backtrack-reorg";
pub const WATCH: &str = "control.watch";
pub const UNWATCH: &str = "control.unwatch";
pub const CLEAN_BLOCKS: &str = "clean-blocks";
pub const CLEAN_FINAL_BLOCKS: &str = "clean-final-blocks";
pub const NEW_EVENT: &str = "new-event";
pub const FINAL_EVENT: &str = "final-event";
// Catchup routing keys.
pub const CATCHUP: &str = "catchup";
pub const RANGE_CATCHUP: &str = "range-catchup";
pub const CATCHUP_EVENT: &str = "catchup-event";

pub fn consumer_new_event_routing(consumer_id: String) -> String {
    format!("{}.{}", consumer_id, NEW_EVENT)
}

pub fn consumer_catchup_event_routing(consumer_id: String) -> String {
    format!("{}.{}", consumer_id, CATCHUP_EVENT)
}

pub fn consumer_final_event_routing(consumer_id: String) -> String {
    format!("{}.{}", consumer_id, FINAL_EVENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consumer_final_event_routing_uses_final_event_key() {
        let routing = consumer_final_event_routing("gateway".into());
        assert_eq!(routing, "gateway.final-event");
    }

    #[test]
    fn consumer_event_routings_do_not_collide() {
        let consumer_id = "gateway";
        let live = consumer_new_event_routing(consumer_id.into());
        let catchup = consumer_catchup_event_routing(consumer_id.into());
        let fin = consumer_final_event_routing(consumer_id.into());
        assert_ne!(fin, live, "final and new-event routings must not collide");
        assert_ne!(
            fin, catchup,
            "final and catchup-event routings must not collide"
        );
    }
}
