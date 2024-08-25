use fhevm_engine_common::generate_fhe_keys;

fn main() {
    let output_dir = "fhevm-keys";
    println!("Generating keys...");
    let keys = generate_fhe_keys();
    println!("Creating directory {output_dir}");
    std::fs::create_dir_all(output_dir).unwrap();
    println!("Creating file {output_dir}/cks");
    std::fs::write(format!("{output_dir}/cks"), keys.client_key).unwrap();
    println!("Creating file {output_dir}/pks");
    std::fs::write(format!("{output_dir}/pks"), keys.compact_public_key).unwrap();
    println!("Creating file {output_dir}/sks");
    std::fs::write(format!("{output_dir}/sks"), keys.server_key).unwrap();
}
