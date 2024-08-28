fhevm/coprocessor.pb.go: ../../proto/coprocessor.proto ../../proto/common.proto
	protoc \
		--proto_path=../../proto/ \
		--go_opt=paths=source_relative \
		--go_opt=Mprotos/coprocessor.proto=github.com/zama-ai/fhevm-backend/fhevm-engine/fhevm-go-coproc \
		--go_opt=Mprotos/common.proto=github.com/zama-ai/fhevm-backend/fhevm-engine/fhevm-go-coproc \
		--go-grpc_out=./fhevm/ --go-grpc_opt=paths=source_relative \
		--go_out=./fhevm/ \
		coprocessor.proto common.proto

.PHONY: build
build: fhevm/coprocessor.pb.go
	cd fhevm && go build .

.PHONY: run
run: fhevm/coprocessor.pb.go
	cd fhevm && go run .

.PHONY: test
test: fhevm/coprocessor.pb.go
	cd fhevm && go test ./...
