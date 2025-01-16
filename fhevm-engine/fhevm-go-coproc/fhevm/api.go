package fhevm

import (
	"context"
	"crypto/ecdsa"
	"database/sql"
	"encoding/binary"
	"errors"
	"fmt"
	"os"
	"sync"
	"time"

	"github.com/ethereum/go-ethereum/common"
	_ "github.com/mattn/go-sqlite3"
	"google.golang.org/grpc"
	"google.golang.org/grpc/metadata"
	"google.golang.org/protobuf/proto"
)

const DEFAULT_FHEVM_COPROCESSOR_SYNC_INTERVAL = "1s"

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

type SignedHandle struct {
	Handle    []byte
	Signature []byte
}

type CoprocessorApi interface {
	CreateSession() CoprocessorSession
	GetStore() ComputationStore
}

type SegmentId int

type ExtraData struct {
	FheRandSeed [32]byte
}

type CoprocessorSession interface {
	Execute(input []byte, ed ExtraData, output []byte) error
	ContractAddress() common.Address
	AclContractAddress() common.Address
	NextSegment() SegmentId
	InvalidateSinceSegment(id SegmentId) SegmentId
	Commit() error
	GetStore() ComputationStore
}

type ComputationStore interface {
	InsertComputation(computation ComputationToInsert) error
	InsertComputationBatch(ciphertexts []ComputationToInsert) error
}

type ApiImpl struct {
	store              *SqliteComputationStore
	address            common.Address
	aclContractAddress common.Address
	flushInterval      time.Duration
}

type SessionImpl struct {
	address            common.Address
	aclContractAddress common.Address
	coprocessorKey     *ecdsa.PrivateKey
	isCommitted        bool
	sessionStore       *SessionComputationStore
}

type ComputationOperand struct {
	IsScalar    bool
	Handle      []byte
	FheUintType FheUintType
}

type ComputationToInsert struct {
	segmentId    SegmentId
	Operation    FheOp
	OutputHandle []byte
	Operands     []ComputationOperand
}

type SessionComputationStore struct {
	underlyingCiphertextStore ComputationStore
	insertedHandles           map[string]int
	invalidatedSegments       map[SegmentId]bool
	inserts                   []ComputationToInsert
	isCommitted               bool
	segmentCount              int
}

type SqliteComputationStore struct {
	dbMutex           sync.Mutex
	dbConn            *sql.DB
	coprocessorUrl    string
	coprocessorApiKey string
	jobChannel        chan bool
}

type handleOffset struct {
	segment int
	index   int
}

type ciphertextSegment struct {
	inserts     []ComputationToInsert
	invalidated bool
}

func (coprocApi *ApiImpl) CreateSession() CoprocessorSession {
	return &SessionImpl{
		address:            coprocApi.address,
		aclContractAddress: coprocApi.aclContractAddress,
		isCommitted:        false,
		sessionStore: &SessionComputationStore{
			isCommitted:               false,
			inserts:                   make([]ComputationToInsert, 0),
			insertedHandles:           make(map[string]int),
			invalidatedSegments:       make(map[SegmentId]bool),
			underlyingCiphertextStore: coprocApi.store,
			segmentCount:              0,
		},
	}
}

func (coprocApi *ApiImpl) GetStore() ComputationStore {
	return coprocApi.store
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

func computationToAsyncComputation(computation ComputationToInsert) AsyncComputation {
	inputs := make([]*AsyncComputationInput, 0, len(computation.Operands))
	for _, operand := range computation.Operands {
		if operand.IsScalar {
			inputs = append(inputs, &AsyncComputationInput{
				Input: &AsyncComputationInput_Scalar{
					Scalar: operand.Handle,
				},
			})
		} else {
			inputs = append(inputs, &AsyncComputationInput{
				Input: &AsyncComputationInput_InputHandle{
					InputHandle: operand.Handle,
				},
			})
		}
	}

	return AsyncComputation{
		Operation:    FheOperation(computation.Operation),
		OutputHandle: computation.OutputHandle,
		Inputs:       inputs,
	}
}

func (dbApi *SqliteComputationStore) InsertComputation(computation ComputationToInsert) error {
	dbApi.dbMutex.Lock()
	defer dbApi.dbMutex.Unlock()

	async_computation := computationToAsyncComputation(computation)
	marshalled, err := proto.Marshal(&async_computation)
	if err != nil {
		return err
	}

	_, err = dbApi.dbConn.Exec(`
	  INSERT OR IGNORE INTO computations(output_handle, payload)
	  VALUES(?, ?)
	`, computation.OutputHandle, marshalled)
	if err != nil {
		return err
	}
	return nil
}

func (dbApi *SqliteComputationStore) InsertComputationBatch(computations []ComputationToInsert) error {
	dbApi.dbMutex.Lock()
	defer dbApi.dbMutex.Unlock()

	for _, comp := range computations {
		async_computation := computationToAsyncComputation(comp)
		marshalled, err := proto.Marshal(&async_computation)
		if err != nil {
			return err
		}

		_, err = dbApi.dbConn.Exec(`
			INSERT OR IGNORE INTO computations(output_handle, payload)
			VALUES(?, ?)
		`, comp.OutputHandle, marshalled)
		if err != nil {
			return err
		}
	}

	// notify channel of new work available
	select {
	case dbApi.jobChannel <- true:
	default:
	}

	return nil
}

func InitCoprocessor() (CoprocessorApi, error) {
	if sqliteDbPath, ok := os.LookupEnv("FHEVM_CIPHERTEXTS_DB"); ok {
		contractAddr, hasAddr := os.LookupEnv("FHEVM_CONTRACT_ADDRESS")
		if !hasAddr {
			return nil, errors.New("FHEVM_CIPHERTEXTS_DB is set but FHEVM_CONTRACT_ADDRESS is not set")
		}
		fhevmContractAddress := common.HexToAddress(contractAddr)
		fmt.Printf("Coprocessor contract address: %s\n", fhevmContractAddress)

		coprocUrl, hasUrl := os.LookupEnv("FHEVM_COPROCESSOR_URL")
		if !hasUrl {
			return nil, errors.New("FHEVM_COPROCESSOR_URL is not configured")
		}

		coprocApiKey, hasApiKey := os.LookupEnv("FHEVM_COPROCESSOR_API_KEY")
		if !hasApiKey {
			return nil, errors.New("FHEVM_COPROCESSOR_API_KEY is not configured")
		}

		ciphertextDb, err := CreateSqliteCiphertextStore(sqliteDbPath, context.Background(), coprocUrl, coprocApiKey)
		if err != nil {
			return nil, err
		}

		aclContractAddressHex := os.Getenv("ACL_CONTRACT_ADDRESS")
		if !common.IsHexAddress(aclContractAddressHex) {
			return nil, fmt.Errorf("bad or missing ACL_CONTRACT_ADDRESS: %s", aclContractAddressHex)
		}
		aclContractAddress := common.HexToAddress(aclContractAddressHex)

		coprocessorSyncIntervalString, isConfigured := os.LookupEnv("FHEVM_COPROCESSOR_SYNC_INTERVAL")
		if !isConfigured {
			coprocessorSyncIntervalString = DEFAULT_FHEVM_COPROCESSOR_SYNC_INTERVAL
		}
		coprocessorSyncInterval, err := time.ParseDuration(coprocessorSyncIntervalString)
		if err != nil {
			return nil, fmt.Errorf("parsing FHEVM_COPROCESSOR_SYNC_INTERVAL (%s): %w", coprocessorSyncIntervalString, err)
		}
		fmt.Printf("Using coprocessor sync interval %s\n", coprocessorSyncInterval.String())

		apiImpl := ApiImpl{
			store:              ciphertextDb,
			address:            fhevmContractAddress,
			aclContractAddress: aclContractAddress,
			flushInterval:      coprocessorSyncInterval,
		}

		// background job to submit computations to coprocessor
		scheduleCoprocessorFlushes(&apiImpl)

		return &apiImpl, nil
	}

	return nil, nil
}

func scheduleCoprocessorFlushes(impl *ApiImpl) {
	// timer to send polling for messages every 10 seconds
	go func() {
		for {
			time.Sleep(impl.flushInterval)
			select {
			case impl.store.jobChannel <- true:
			default:
			}
		}
	}()

	// listen to new jobs, scheduled manually or by timer
	go func() {
		for {
			<-impl.store.jobChannel
			itemsComputed, err := flushWorkItemsToCoprocessor(impl.store)
			if err != nil {
				fmt.Printf("error flushing work items to coprocessor: %s\n", err)
			} else if itemsComputed > 0 {
				fmt.Printf("successfully sent %d work items to the coprocessor\n", itemsComputed)
			}
		}
	}()
}

func queryCurrentComputeRequests(store *SqliteComputationStore) (*AsyncComputeRequest, [][]byte, error) {
	var asyncCompReq *AsyncComputeRequest
	handlesToMarkDone := make([][]byte, 0)

	store.dbMutex.Lock()
	defer store.dbMutex.Unlock()

	// query all ciphertexts
	response, err := store.dbConn.Query("SELECT output_handle, payload FROM computations WHERE is_sent = 0 LIMIT 700")
	if err != nil {
		return nil, handlesToMarkDone, err
	}

	requests := make([]*AsyncComputation, 0, 16)
	for response.Next() {
		var outputHandle, payload []byte
		err = response.Scan(&outputHandle, &payload)
		if err != nil {
			return nil, handlesToMarkDone, err
		}
		var ac AsyncComputation
		err = proto.Unmarshal(payload, &ac)
		if err != nil {
			return nil, handlesToMarkDone, err
		}

		requests = append(requests, &ac)
		handlesToMarkDone = append(handlesToMarkDone, outputHandle)
	}

	if response.Err() != nil {
		return nil, handlesToMarkDone, err
	}

	if len(requests) > 0 {
		asyncCompReq = &AsyncComputeRequest{
			Computations: requests,
		}
	}

	return asyncCompReq, handlesToMarkDone, nil
}

func flushWorkItemsToCoprocessor(store *SqliteComputationStore) (int, error) {
	asyncCompReq, handlesToMarkDone, err := queryCurrentComputeRequests(store)
	if err != nil {
		return 0, err
	}

	if asyncCompReq != nil {
		// send the request
		var opts []grpc.DialOption
		opts = append(opts, grpc.WithInsecure())
		conn, err := grpc.NewClient(store.coprocessorUrl, opts...)
		if err != nil {
			return 0, err
		}
		defer conn.Close()

		fmt.Printf("Sending %d computations to coproc\n", len(asyncCompReq.Computations))

		client := NewFhevmCoprocessorClient(conn)
		md := metadata.Pairs("Authorization", fmt.Sprintf("Bearer %s", store.coprocessorApiKey))
		grpcContext := metadata.NewOutgoingContext(context.Background(), md)
		_, err = client.AsyncCompute(grpcContext, asyncCompReq)
		if err != nil {
			return 0, err
		}

		// mark handles done in db
		store.dbMutex.Lock()
		defer store.dbMutex.Unlock()
		stmt, err := store.dbConn.Prepare("UPDATE computations SET is_sent = 1 WHERE output_handle = ?")
		if err != nil {
			return 0, err
		}

		for _, handle := range handlesToMarkDone {
			_, err := stmt.Exec(handle)
			if err != nil {
				return 0, err
			}
		}

		return len(asyncCompReq.Computations), nil
	}

	return 0, nil
}

func CreateSqliteCiphertextStore(dbPath string, ctx context.Context, coprocUrl, coprocApiKey string) (*SqliteComputationStore, error) {
	dbConn, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, err
	}

	err = doMigrations(dbConn, ctx)
	if err != nil {
		return nil, err
	}

	return &SqliteComputationStore{
		dbConn:            dbConn,
		dbMutex:           sync.Mutex{},
		coprocessorUrl:    coprocUrl,
		coprocessorApiKey: coprocApiKey,
		jobChannel:        make(chan bool),
	}, nil
}

func doMigrations(dbConn *sql.DB, ctx context.Context) error {
	trx, err := dbConn.BeginTx(ctx, &sql.TxOptions{})
	if err != nil {
		return err
	}

	_, err = trx.Exec(`
		CREATE TABLE IF NOT EXISTS computations(
			output_handle BINARY PRIMARY KEY,
			payload BINARY, -- protobuf AsyncComputation type
			is_sent INT DEFAULT 0
		);

		CREATE INDEX IF NOT EXISTS computations_sent ON computations(is_sent);
	`)
	if err != nil {
		return err
	}

	err = trx.Commit()
	if err != nil {
		return err
	}

	return nil
}
