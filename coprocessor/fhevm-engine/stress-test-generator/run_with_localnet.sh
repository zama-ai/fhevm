source ./../.env-test


cargo run --jobs 32 --release $FEATURES -- \
--run-server \
--listen-address=0.0.0.0:3030
