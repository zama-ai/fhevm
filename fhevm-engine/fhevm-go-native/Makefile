fhevm/executor.pb.go: ../../proto/executor.proto ../../proto/common.proto
	protoc \
		--proto_path=../../proto/ \
		--go_opt=paths=source_relative \
		--go_opt=Mprotos/executor.proto=github.com/zama-ai/fhevm-backend/fhevm-engine/fhevm-go-native \
		--go_opt=Mprotos/common.proto=github.com/zama-ai/fhevm-backend/fhevm-engine/fhevm-go-native \
		--go-grpc_out=./fhevm/ --go-grpc_opt=paths=source_relative \
		--go_out=./fhevm/ \
		executor.proto common.proto

.PHONY: build
build: fhevm/executor.pb.go
	cd fhevm && go build .

.PHONY: run
run: fhevm/executor.pb.go
	cd fhevm && go run .

.PHONY: test
test: fhevm/executor.pb.go
	cd fhevm && go test ./...
