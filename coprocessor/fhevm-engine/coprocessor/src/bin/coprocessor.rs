fn main() {
    let args = coprocessor::daemon_cli::parse_args();
    assert!(
        args.work_items_batch_size < args.tenant_key_cache_size,
        "Work items batch size must be less than tenant key cache size"
    );

    if args.generate_fhe_keys {
        coprocessor::generate_dump_fhe_keys();
    } else {
        coprocessor::start_runtime(args, None);
    }
}
