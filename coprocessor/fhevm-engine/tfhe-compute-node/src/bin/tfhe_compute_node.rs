fn main() {
    let args = tfhe_compute_node::cli::parse();
    //TODO: add signal handler and pass close_recv
    tfhe_compute_node::start_runtime(args, None);
}
