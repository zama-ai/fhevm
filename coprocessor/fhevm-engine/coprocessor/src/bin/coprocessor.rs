fn main() {
    let args = coprocessor::daemon_cli::parse_args();
    if args.generate_fhe_keys {
        coprocessor::generate_dump_fhe_keys();
    } else {
        coprocessor::start_runtime(args, None);
    }
}
