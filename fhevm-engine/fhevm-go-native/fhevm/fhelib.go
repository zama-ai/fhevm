package fhevm

import (
	"encoding/binary"
	"fmt"

	"github.com/ethereum/go-ethereum/crypto"
)

type FheLibMethod struct {
	// Name of the fhelib function
	Name string
	// types of the arguments that the fhelib function take. format is "(type1,type2...)" (e.g "(uint256,bytes1)")
	ArgTypes          string
	runFunction       func(sess ExecutorSession, input []byte, ed ExtraData, outputHandle []byte) error
	ScalarSupport     bool
	NonScalarDisabled bool
}

func (m FheLibMethod) String() string {
	return fmt.Sprintf(
		"FheLibMethod(Name: %s, ArgTypes: %s, ScalarSupport: %t, NonScalarDisabled: %t)",
		m.Name, m.ArgTypes, m.ScalarSupport, m.NonScalarDisabled,
	)
}

var signatureToFheLibMethod = map[uint32]*FheLibMethod{}

func FheLibMethods() []*FheLibMethod {
	return []*FheLibMethod{
		{
			Name:          "fheAdd",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheAddRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheSub",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheSubRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheMul",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheMulRun,
			ScalarSupport: true,
		},
		{
			Name:              "fheRem",
			ArgTypes:          "(uint256,uint256,bytes1)",
			runFunction:       fheRemRun,
			ScalarSupport:     true,
			NonScalarDisabled: true,
		},
		{
			Name:          "fheBitAnd",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheBitAndRun,
			ScalarSupport: false,
		},
		{
			Name:          "fheBitOr",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheBitOrRun,
			ScalarSupport: false,
		},
		{
			Name:          "fheBitXor",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheBitXorRun,
			ScalarSupport: false,
		},
		{
			Name:          "fheShl",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheShlRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheShr",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheShrRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheRotl",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheRotlRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheRotr",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheRotrRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheEq",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheEqRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheNe",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheNeRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheEq",
			ArgTypes:      "(uint256,bytes,bytes1)",
			runFunction:   fheEqBytesRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheNe",
			ArgTypes:      "(uint256,bytes,bytes1)",
			runFunction:   fheNeBytesRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheGe",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheGeRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheGt",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheGtRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheLe",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheLeRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheLt",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheLtRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheMin",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheMinRun,
			ScalarSupport: true,
		},
		{
			Name:          "fheMax",
			ArgTypes:      "(uint256,uint256,bytes1)",
			runFunction:   fheMaxRun,
			ScalarSupport: true,
		},
		{
			Name:        "fheNeg",
			ArgTypes:    "(uint256)",
			runFunction: fheNegRun,
		},
		{
			Name:        "fheNot",
			ArgTypes:    "(uint256)",
			runFunction: fheNotRun,
		},
		{
			Name:        "fheIfThenElse",
			ArgTypes:    "(uint256,uint256,uint256)",
			runFunction: fheIfThenElseRun,
		},
		{
			Name:        "cast",
			ArgTypes:    "(uint256,bytes1)",
			runFunction: castRun,
		},
		{
			Name:        "fheRand",
			ArgTypes:    "(bytes1)",
			runFunction: fheRandRun,
		},
		{
			Name:        "fheRandBounded",
			ArgTypes:    "(uint256,bytes1)",
			runFunction: fheRandBoundedRun,
		},
		{
			Name:        "trivialEncrypt",
			ArgTypes:    "(uint256,bytes1)",
			runFunction: trivialEncryptRun,
		},
		{
			Name:        "trivialEncrypt",
			ArgTypes:    "(bytes,bytes1)",
			runFunction: trivialEncryptBytesRun,
		},
	}
}

func MakeKeccakSignature(input string) uint32 {
	return binary.BigEndian.Uint32(crypto.Keccak256([]byte(input))[0:4])
}

func init() {
	// create the mapping for every available fhelib method
	for _, method := range FheLibMethods() {
		signature := fmt.Sprintf("%s%s", method.Name, method.ArgTypes)
		signatureNum := MakeKeccakSignature(signature)
		signatureToFheLibMethod[signatureNum] = method
	}
}
