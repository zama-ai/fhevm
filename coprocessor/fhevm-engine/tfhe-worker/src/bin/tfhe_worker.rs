fn main() {
    let args = tfhe_worker::daemon_cli::parse_args();
    if args.generate_fhe_keys {
        tfhe_worker::generate_dump_fhe_keys();
    } else {
        tfhe_worker::start_runtime(args, None);
    }
}
