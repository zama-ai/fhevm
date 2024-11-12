package fhevm

import (
	"encoding/binary"
	"errors"
	"fmt"
	"math/big"
	"os"
	"sort"

	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	_ "github.com/mattn/go-sqlite3"
)

type FheUintType uint8

const (
	FheBool      FheUintType = 0
	FheUint4     FheUintType = 1
	FheUint8     FheUintType = 2
	FheUint16    FheUintType = 3
	FheUint32    FheUintType = 4
	FheUint64    FheUintType = 5
	FheUint128   FheUintType = 6
	FheUint160   FheUintType = 7
	FheUint256   FheUintType = 8
	FheEbytes64  FheUintType = 9
	FheEbytes128 FheUintType = 10
	FheEbytes256 FheUintType = 11
	FheUserBytes FheUintType = 255
)

type FheOp uint8

const (
	FheAdd    FheOp = 0
	FheSub    FheOp = 1
	FheMul    FheOp = 2
	FheDiv    FheOp = 3
	FheRem    FheOp = 4
	FheBitAnd FheOp = 5
	FheBitOr  FheOp = 6
	FheBitXor FheOp = 7
	FheShl    FheOp = 8
	FheShr    FheOp = 9
	FheRotl   FheOp = 10
	FheRotr   FheOp = 11
	FheEq     FheOp = 12
	FheNe     FheOp = 13
	FheGe     FheOp = 14
	FheGt     FheOp = 15
	FheLe     FheOp = 16
	FheLt     FheOp = 17
	FheMin    FheOp = 18
	FheMax    FheOp = 19
	FheNeg    FheOp = 20
	FheNot    FheOp = 21
	// unused
	// VerifyCiphertext FheOp = 22
	FheCast FheOp = 23
	// unused
	TrivialEncrypt FheOp = 24
	FheIfThenElse  FheOp = 25
	FheRand        FheOp = 26
	FheRandBounded FheOp = 27
)

func (t FheUintType) String() string {
	switch t {
	case FheBool:
		return "fheBool"
	case FheUint4:
		return "fheUint4"
	case FheUint8:
		return "fheUint8"
	case FheUint16:
		return "fheUint16"
	case FheUint32:
		return "fheUint32"
	case FheUint64:
		return "fheUint64"
	case FheUint128:
		return "fheUint128"
	case FheUint160:
		return "fheUint160"
	case FheUint256:
		return "fheUint256"
	case FheEbytes64:
		return "fheEbytes64"
	case FheEbytes128:
		return "fheEbytes128"
	case FheEbytes256:
		return "fheEbytes256"
	default:
		return "unknownFheUintType"
	}
}

func IsValidFheType(t byte) bool {
	if uint8(t) < uint8(FheBool) || uint8(t) > uint8(FheEbytes256) {
		return false
	}
	return true
}

// Api to the storage of the host chain, must be passed
// from the EVM to us
type ChainStorageApi interface {
	GetState(common.Address, common.Hash) common.Hash
	SetState(common.Address, common.Hash, common.Hash)
}

type ExecutorApi interface {
	// Create a session for a single transaction to capture all fhe
	// operations inside the state. We also schedule asynchronous
	// compute in background to have operations inside
	// the cache prepared to be inserted when commit block comes.
	// We pass current block number to know at which
	// block ciphertext should be materialized inside blockchain state.
	CreateSession(blockNumber int64, storage ChainStorageApi) ExecutorSession
	// Insert existing fhe operations to the state from inside the state
	// storage queue. This should be called at the end of every block.
	FlushFheResultsToState(blockNumber int64, storage ChainStorageApi) ExecutorSession
}

type SegmentId int

type ExtraData struct {
	FheRandSeed [32]byte
}

type ExecutorSession interface {
	Execute(input []byte, ed ExtraData, output []byte) error
	ContractAddress() common.Address
	AclContractAddress() common.Address
	NextSegment() SegmentId
	InvalidateSinceSegment(id SegmentId) SegmentId
	// After commit fhe computations will be put inside the queue
	// to the blockchain state
	Commit() error
	GetStore() ComputationStore
}

type ComputationStore interface {
	InsertComputation(computation ComputationToInsert) error
	InsertComputationBatch(ciphertexts []ComputationToInsert) error
}

type ApiImpl struct {
	address                common.Address
	aclContractAddress     common.Address
	contractStorageAddress common.Address
}

type SessionImpl struct {
	address            common.Address
	aclContractAddress common.Address
	isCommitted        bool
	sessionStore       *SessionComputationStore
	storage            ChainStorageApi
}

type ComputationOperand struct {
	IsScalar    bool
	Handle      []byte
	FheUintType FheUintType
}

type ComputationToInsert struct {
	segmentId     SegmentId
	Operation     FheOp
	OutputHandle  []byte
	Operands      []ComputationOperand
	CommitBlockId int64
}

type SessionComputationStore struct {
	underlyingCiphertextStore ComputationStore
	insertedHandles           map[string]int
	invalidatedSegments       map[SegmentId]bool
	inserts                   []ComputationToInsert
	isCommitted               bool
	segmentCount              int
	blockNumber               int64
}

type EvmStorageComputationStore struct {
	evmStorage             ChainStorageApi
	currentBlockNumber     int64
	contractStorageAddress common.Address
}

type handleOffset struct {
	segment int
	index   int
}

type ciphertextSegment struct {
	inserts     []ComputationToInsert
	invalidated bool
}

func (executorApi *ApiImpl) CreateSession(blockNumber int64, api ChainStorageApi) ExecutorSession {
	return &SessionImpl{
		address:            executorApi.address,
		aclContractAddress: executorApi.aclContractAddress,
		isCommitted:        false,
		sessionStore: &SessionComputationStore{
			isCommitted:         false,
			inserts:             make([]ComputationToInsert, 0),
			insertedHandles:     make(map[string]int),
			invalidatedSegments: make(map[SegmentId]bool),
			segmentCount:        0,
			blockNumber:         blockNumber,
			underlyingCiphertextStore: &EvmStorageComputationStore{
				evmStorage:             api,
				contractStorageAddress: executorApi.contractStorageAddress,
				currentBlockNumber:     blockNumber,
			},
		},
	}
}

func (sessionApi *SessionImpl) Commit() error {
	if sessionApi.isCommitted {
		return errors.New("session is already comitted")
	}

	err := sessionApi.sessionStore.Commit()
	if err != nil {
		return err
	}

	return nil
}

func (sessionApi *SessionImpl) Execute(dataOrig []byte, ed ExtraData, outputOrig []byte) error {
	if len(dataOrig) < 4 {
		return fmt.Errorf("input data must be at least 4 bytes for signature, got %d", len(dataOrig))
	}

	// make copies so we could assume array is immutable later
	data := make([]byte, len(dataOrig))
	output := make([]byte, len(outputOrig))
	copy(data, dataOrig)
	copy(output, outputOrig)

	signature := binary.BigEndian.Uint32(data[0:4])
	callData := data[4:]

	method, exists := signatureToFheLibMethod[signature]
	if exists {
		fmt.Printf("Executing captured operation %s%s\n", method.Name, method.ArgTypes)
		if len(output) >= 32 {
			// where to get output handle from?
			outputHandle := output[0:32]
			return method.runFunction(sessionApi, callData, ed, outputHandle)
		} else {
			return errors.New("no output data provided")
		}
	} else {
		return fmt.Errorf("signature %d not recognized", signature)
	}
}

func (sessionApi *SessionImpl) NextSegment() SegmentId {
	sessionApi.sessionStore.segmentCount = sessionApi.sessionStore.segmentCount + 1
	return SegmentId(sessionApi.sessionStore.segmentCount)
}

func (sessionApi *SessionImpl) InvalidateSinceSegment(id SegmentId) SegmentId {
	for idx := int(id); idx <= sessionApi.sessionStore.segmentCount; idx++ {
		sessionApi.sessionStore.invalidatedSegments[SegmentId(idx)] = true
	}

	return sessionApi.NextSegment()
}

func (sessionApi *SessionImpl) ContractAddress() common.Address {
	return sessionApi.address
}

func (sessionApi *SessionImpl) AclContractAddress() common.Address {
	return sessionApi.aclContractAddress
}

func (sessionApi *SessionImpl) GetStore() ComputationStore {
	return sessionApi.sessionStore
}

func (dbApi *SessionComputationStore) InsertComputationBatch(computations []ComputationToInsert) error {
	for _, comp := range computations {
		dbApi.InsertComputation(comp)
	}

	return nil
}

func (dbApi *SessionComputationStore) InsertComputation(computation ComputationToInsert) error {
	_, found := dbApi.insertedHandles[string(computation.OutputHandle)]
	if !found {
		// preserve insertion order
		dbApi.insertedHandles[string(computation.OutputHandle)] = len(dbApi.inserts)
		computation.segmentId = SegmentId(dbApi.segmentCount)
		// hardcode late commit for now to be 5 blocks from current block
		// in future we can implement dynamic compute, if user pays more
		// he can have faster commit
		computation.CommitBlockId = dbApi.blockNumber + 5
		dbApi.inserts = append(dbApi.inserts, computation)
	}

	return nil
}

func (dbApi *SessionComputationStore) Commit() error {
	if dbApi.isCommitted {
		return errors.New("session computation store already committed")
	}

	dbApi.isCommitted = true

	finalInserts := make([]ComputationToInsert, 0, len(dbApi.inserts))
	for _, ct := range dbApi.inserts {
		if !dbApi.invalidatedSegments[ct.segmentId] {
			finalInserts = append(finalInserts, ct)
		}
	}

	fmt.Printf("Inserting %d computations into database\n", len(finalInserts))

	err := dbApi.underlyingCiphertextStore.InsertComputationBatch(finalInserts)
	if err != nil {
		return err
	}

	return nil
}

func blockNumberToQueueItemCountAddress(blockNumber int64) common.Hash {
	return common.BigToHash(big.NewInt(blockNumber))
}

func blockQueueStorageLayout(blockNumber int64, ctNumber int64) NativeQueueAddressLayout {
	toHash := common.BigToHash(big.NewInt(blockNumber))
	// main storage prefix
	// number is on the right bitwise, should never overwrite storage prefix
	// because block numbers are much less than 256 bit numbers
	copy(toHash[:], "main")
	initialOffsetHash := crypto.Keccak256(toHash[:])
	copy(toHash[:], "bigscalar")
	bigScalarOffsetHash := crypto.Keccak256(toHash[:])
	bigScalarNum := new(big.Int)
	bigScalarNum.SetBytes(bigScalarOffsetHash)
	// 2048 bit is maximum supported number
	// one 2048 bit contains 8 256 bit words
	bigScalarNum.Add(bigScalarNum, big.NewInt(ctNumber*8))

	one := big.NewInt(1)
	res := new(big.Int)
	res.SetBytes(initialOffsetHash)
	// four 256 bit words, calculate offset
	// according to ciphertext number
	res.Add(res, big.NewInt(ctNumber*4))
	metadata := common.BytesToHash(res.Bytes())
	res.Add(res, one)
	outputHandle := common.BytesToHash(res.Bytes())
	res.Add(res, one)
	firstOperand := common.BytesToHash(res.Bytes())
	res.Add(res, one)
	secondOperand := common.BytesToHash(res.Bytes())
	res.Add(res, one)
	return NativeQueueAddressLayout{
		metadata:         metadata,
		outputHandle:     outputHandle,
		firstOperand:     firstOperand,
		secondOperand:    secondOperand,
		bigScalarOperand: common.Hash(bigScalarOffsetHash),
	}
}

func computationMetadata(comp ComputationToInsert) common.Hash {
	var res common.Hash

	// operation type
	res[0] = byte(comp.Operation)
	for _, op := range comp.Operands {
		if op.IsScalar {
			// set scalar byte
			res[1] = 1
			if op.FheUintType > FheUint256 {
				// set big scalar byte, we'll need big scalar register
				// for this computation
				res[2] = 1
			}
		}
	}

	return res
}

func bytesToMetadata(input common.Hash) ComputationMetadata {
	return ComputationMetadata{
		Operation:   FheOp(input[0]),
		IsScalar:    input[1] > 0,
		IsBigScalar: input[2] > 0,
	}
}

type ComputationMetadata struct {
	Operation   FheOp
	IsScalar    bool
	IsBigScalar bool
}

type NativeQueueAddressLayout struct {
	// metadata about the computation
	// like operation type, is scalar etc
	metadata common.Hash
	// output handle of the computation
	outputHandle common.Hash
	// first operand to the computation
	firstOperand common.Hash
	// second operand to the computation
	secondOperand common.Hash
	// if operand size is more than 256 bits
	// it is stored in special place here
	bigScalarOperand common.Hash
}

func (dbApi *EvmStorageComputationStore) InsertComputationBatch(computations []ComputationToInsert) error {
	// storage layout for the late commit queue:
	//
	// blockNumber address - stores the amount of ciphertexts in the queue in the block,
	// block number is directly converted to storage address which has count for the queue
	// blockNumber represents when ciphertexts are to be commited to the storage
	// and queue should be cleaned up after the block passes
	//
	// queue address - hash 'main' prefix and block number converted to 32 big endian bytes
	// this address contains all the handles to be computed in this block
	// example:
	// keccak256('main' .. blockNumber) + 0 - operation metadata, is extended scalar operand needed
	// keccak256('main' .. blockNumber) + 1 - output ciphertext handle
	// keccak256('main' .. blockNumber) + 2 - first ciphertext argument
	// keccak256('main' .. blockNumber) + 3 - second ciphertext argument
	//
	// if scalar operand is bigger than 256 bit number, we use special
	// bigscalar address

	// prepare for dynamic evaluation. Say, users want to evaluate ciphertext
	// in 5 or 10 blocks from current block, depending on how much they pay.
	// We create buckets, how many blocks in the future user wants
	// his ciphertexts to be evaluated
	buckets := make(map[int64][]*ComputationToInsert)
	// index the buckets
	for _, comp := range computations {
		if buckets[comp.CommitBlockId] == nil {
			buckets[comp.CommitBlockId] = make([]*ComputationToInsert, 0)
		}
		buckets[comp.CommitBlockId] = append(buckets[comp.CommitBlockId], &comp)
	}
	// collect all their keys and sort because golang doesn't traverse map
	// in deterministic order
	allKeys := make([]int, 0)
	for k, _ := range buckets {
		allKeys = append(allKeys, int(k))
	}
	sort.Ints(allKeys)

	// iterate all buckets and put items to their appropriate block queues
	for _, key := range allKeys {
		queueBlockNumber := int64(key)
		bucket := buckets[queueBlockNumber]

		countAddress := blockNumberToQueueItemCountAddress(queueBlockNumber)
		ciphertextsInBlock := dbApi.evmStorage.GetState(dbApi.contractStorageAddress, countAddress).Big()
		one := big.NewInt(1)

		for idx, comp := range bucket {
			layout := blockQueueStorageLayout(queueBlockNumber, idx)
			ciphertextsInBlock = ciphertextsInBlock.Add(ciphertextsInBlock, one)
			metadata := computationMetadata(*comp)
			dbApi.evmStorage.SetState(dbApi.contractStorageAddress, layout.metadata, metadata)
			dbApi.evmStorage.SetState(dbApi.contractStorageAddress, layout.outputHandle, common.Hash(comp.OutputHandle))
			if len(comp.Operands) > 0 {
				dbApi.evmStorage.SetState(dbApi.contractStorageAddress, layout.firstOperand, common.Hash(comp.Operands[0].Handle))
			}
			if len(comp.Operands) > 1 {
				dbApi.evmStorage.SetState(dbApi.contractStorageAddress, layout.secondOperand, common.Hash(comp.Operands[1].Handle))
			}
		}

		// set updated count back
		dbApi.evmStorage.SetState(dbApi.contractStorageAddress, countAddress, common.BigToHash(ciphertextsInBlock))
	}

	return nil
}

func (executorApi *ApiImpl) FlushFheResultsToState(blockNumber int64, api ChainStorageApi) ExecutorSession {
	// cleanup the queue for the block number
	countAddress := blockNumberToQueueItemCountAddress(blockNumber)
	ciphertextsInBlock := api.GetState(executorApi.contractStorageAddress, countAddress).Big()
	ctCount := ciphertextsInBlock.Int64()
	zero := common.BigToHash(big.NewInt(0))
	one := big.NewInt(1)

	// zero out queue ciphertexts
	for i := 0; i < int(ctCount); i++ {
		ctNumber := big.NewInt(int64(i))
		ctAddr := blockQueueStorageLayout(blockNumber, ctNumber)
		metadata := bytesToMetadata(api.GetState(executorApi.contractStorageAddress, ctAddr.metadata))
		api.SetState(executorApi.contractStorageAddress, ctAddr.metadata, zero)
		api.SetState(executorApi.contractStorageAddress, ctAddr.outputHandle, zero)
		api.SetState(executorApi.contractStorageAddress, ctAddr.firstOperand, zero)
		api.SetState(executorApi.contractStorageAddress, ctAddr.secondOperand, zero)
		if metadata.IsBigScalar {
			counter := new(big.Int)
			counter.SetBytes(ctAddr.bigScalarOperand[:])
			// max supporter number 2048 is 2048
			for i := 0; i < 2048/256; i++ {
				api.SetState(executorApi.contractStorageAddress, common.BigToHash(counter), zero)
				counter.Add(counter, one)
			}
		}
	}

	// set 0 as count
	api.SetState(executorApi.contractStorageAddress, countAddress, zero)

	panic("TODO: implement flushing of ciphertext data to the blockchain state")
}

func (dbApi *EvmStorageComputationStore) InsertComputation(computation ComputationToInsert) error {
	return dbApi.InsertComputationBatch([]ComputationToInsert{computation})
}

func (dbApi *EvmStorageComputationStore) Commit() error {
	// no commit inside EVM state store
	return nil
}

func InitExecutor() (ExecutorApi, error) {
	contractAddr, hasAddr := os.LookupEnv("FHEVM_CONTRACT_ADDRESS")
	if !hasAddr {
		return nil, errors.New("FHEVM_CIPHERTEXTS_DB is set but FHEVM_CONTRACT_ADDRESS is not set")
	}
	fhevmContractAddress := common.HexToAddress(contractAddr)
	fmt.Printf("Coprocessor contract address: %s\n", fhevmContractAddress)

	aclContractAddressHex := os.Getenv("ACL_CONTRACT_ADDRESS")
	if !common.IsHexAddress(aclContractAddressHex) {
		return nil, fmt.Errorf("bad or missing ACL_CONTRACT_ADDRESS: %s", aclContractAddressHex)
	}
	aclContractAddress := common.HexToAddress(aclContractAddressHex)

	// pick hardcoded value in the beginning, we can change later
	storageAddress := common.HexToAddress("0x0000000000000000000000000000000000000070")
	apiImpl := ApiImpl{
		address:                fhevmContractAddress,
		aclContractAddress:     aclContractAddress,
		contractStorageAddress: storageAddress,
	}

	return &apiImpl, nil
}
