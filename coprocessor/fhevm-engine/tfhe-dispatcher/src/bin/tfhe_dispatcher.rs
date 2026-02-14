fn main() {
    let args = tfhe_dispatcher::cli::parse_args();
    //TODO: add signal handler and pass close_recv
    tfhe_dispatcher::start_runtime(args, None);
}
