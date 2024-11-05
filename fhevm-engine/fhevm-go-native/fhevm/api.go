package fhevm

import (
	"encoding/binary"
	"errors"
	"fmt"
	"os"

	"github.com/ethereum/go-ethereum/common"
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
	address            common.Address
	aclContractAddress common.Address
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
	evmStorage ChainStorageApi
}

type handleOffset struct {
	segment int
	index   int
}

type ciphertextSegment struct {
	inserts     []ComputationToInsert
	invalidated bool
}

func (coprocApi *ApiImpl) CreateSession(blockNumber int64, api ChainStorageApi) ExecutorSession {
	return &SessionImpl{
		address:            coprocApi.address,
		aclContractAddress: coprocApi.aclContractAddress,
		isCommitted:        false,
		sessionStore: &SessionComputationStore{
			isCommitted:               false,
			inserts:                   make([]ComputationToInsert, 0),
			insertedHandles:           make(map[string]int),
			invalidatedSegments:       make(map[SegmentId]bool),
			segmentCount:              0,
			blockNumber:               blockNumber,
			underlyingCiphertextStore: &EvmStorageComputationStore{evmStorage: api},
		},
	}
}

func (coprocApi *ApiImpl) FlushFheResultsToState(blockNumber int64, api ChainStorageApi) ExecutorSession {
	panic("TODO: implement flushing to the blockchain state")
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

func (dbApi *EvmStorageComputationStore) InsertComputationBatch(computations []ComputationToInsert) error {
	for _, comp := range computations {
		dbApi.InsertComputation(comp)
	}

	return nil
}

func (dbApi *EvmStorageComputationStore) InsertComputation(computation ComputationToInsert) error {
	panic("TODO: implement insert computation to EVM")
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

	apiImpl := ApiImpl{
		address:            fhevmContractAddress,
		aclContractAddress: aclContractAddress,
	}

	return &apiImpl, nil
}
