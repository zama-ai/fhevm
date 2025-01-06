package fhevm

import (
	"errors"
	"fmt"
	"math/big"
)

func handleType(handle []byte) FheUintType {
	return FheUintType(handle[30])
}

func fheAddRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheAdd
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})

		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})

		if err != nil {
			return err
		}
		return nil
	}
}

func fheSubRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheSub
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})

		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})

		if err != nil {
			return err
		}
		return nil
	}
}

func fheDivRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	var lhs, rhs []byte
	if isScalar {
		lhs, rhs, err = getScalarOperands(sess, input)
	} else {
		lhs, rhs, err = get2FheOperands(sess, input)
	}
	if err != nil {
		return err
	}

	computation := ComputationToInsert{
		Operation:    FheDiv,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      lhs,
				FheUintType: handleType(lhs),
				IsScalar:    false,
			},
			{
				Handle:      rhs,
				FheUintType: handleType(rhs),
				IsScalar:    isScalar,
			},
		},
	}

	return sess.GetStore().InsertComputation(computation)
}

func fheMulRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheMul
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})

		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})

		if err != nil {
			return err
		}
		return nil
	}
}

func fheRemRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheRem
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})

		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheBitAndRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheBitAnd
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		return errors.New("scalar fheBitAnd is not supported")
	}
}

func fheBitOrRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheBitOr
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		return errors.New("scalar fheBitOr is not supported")
	}
}

func fheBitXorRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheBitXor
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		return errors.New("scalar fheBitXor is not supported")
	}
}

func fheShlRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheShl
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheShrRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheShr
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheRotlRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheRotl
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheRotrRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheRotr
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheEqRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheEq
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheNeRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheNe
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheEqBytesRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	// uint256,bytes,bytes1
	if len(unslicedInput) < 128 {
		return fmt.Errorf("expected at least 128 bytes as input, got %d", len(unslicedInput))
	}
	lhs := unslicedInput[0:32]
	lhsByteOffset := unslicedInput[32:64]
	isScalar := unslicedInput[64] > 0

	lhsByteOffsetNum := big.NewInt(0)
	lhsByteOffsetNum.SetBytes(lhsByteOffset)

	offsetEnd := lhsByteOffsetNum.Uint64() + 32
	if offsetEnd > uint64(len(unslicedInput)) {
		return fmt.Errorf("byte array offset out of bounds, got %d, input length %d", offsetEnd, len(unslicedInput))
	}

	scalarOperandLengthBytes := unslicedInput[lhsByteOffsetNum.Uint64() : lhsByteOffsetNum.Uint64()+32]
	scalarOperandLength := big.NewInt(0)
	scalarOperandLength.SetBytes(scalarOperandLengthBytes)

	byteArrayEnd := offsetEnd + scalarOperandLength.Uint64()
	if byteArrayEnd > uint64(len(unslicedInput)) {
		return fmt.Errorf("byte array offset out of bounds, got %d, input length %d", byteArrayEnd, len(unslicedInput))
	}

	rhs := unslicedInput[offsetEnd : offsetEnd+scalarOperandLength.Uint64()]

	operation := FheEq
	if isScalar {
		err := sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	} else {
		return errors.New("only scalar is operand supported for fheEq(uint256,bytes,bytes1) overload")
	}
}

func fheNeBytesRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	// uint256,bytes,bytes1
	if len(unslicedInput) < 128 {
		return fmt.Errorf("expected at least 128 bytes as input, got %d", len(unslicedInput))
	}
	lhs := unslicedInput[0:32]
	lhsByteOffset := unslicedInput[32:64]
	isScalar := unslicedInput[64] > 0

	lhsByteOffsetNum := big.NewInt(0)
	lhsByteOffsetNum.SetBytes(lhsByteOffset)

	offsetEnd := lhsByteOffsetNum.Uint64() + 32
	if offsetEnd > uint64(len(unslicedInput)) {
		return fmt.Errorf("byte array offset out of bounds, got %d, input length %d", offsetEnd, len(unslicedInput))
	}

	scalarOperandLengthBytes := unslicedInput[lhsByteOffsetNum.Uint64() : lhsByteOffsetNum.Uint64()+32]
	scalarOperandLength := big.NewInt(0)
	scalarOperandLength.SetBytes(scalarOperandLengthBytes)

	byteArrayEnd := offsetEnd + scalarOperandLength.Uint64()
	if byteArrayEnd > uint64(len(unslicedInput)) {
		return fmt.Errorf("byte array offset out of bounds, got %d, input length %d", byteArrayEnd, len(unslicedInput))
	}

	rhs := unslicedInput[offsetEnd : offsetEnd+scalarOperandLength.Uint64()]

	operation := FheNe
	if isScalar {
		err := sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	} else {
		return errors.New("only scalar is operand supported for fheNe(uint256,bytes,bytes1) overload")
	}
}

func fheGeRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheGe
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheGtRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheGt
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheLeRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheLe
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheLtRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheLt
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheMinRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheMin
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheMaxRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 65 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:65]

	isScalar, err := isScalarOp(input)
	if err != nil {
		return err
	}

	operation := FheMax
	if !isScalar {
		lhs, rhs, err := get2FheOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}

		return nil
	} else {
		lhs, rhs, err := getScalarOperands(sess, input)
		if err != nil {
			return err
		}

		err = sess.GetStore().InsertComputation(ComputationToInsert{
			Operation:    operation,
			OutputHandle: outputHandle,
			Operands: []ComputationOperand{
				{
					Handle:      lhs,
					FheUintType: handleType(lhs),
					IsScalar:    false,
				},
				{
					Handle:      rhs,
					FheUintType: handleType(rhs),
					IsScalar:    isScalar,
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	}
}

func fheNegRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 32 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:32]

	ct, err := getSingleFheOperand(sess, input)
	if err != nil {
		return err
	}

	operation := FheNeg
	err = sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    operation,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      ct,
				FheUintType: handleType(ct),
				IsScalar:    false,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func fheNotRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 32 {
		return fmt.Errorf("expected at least 65 bytes as input, got %d", len(unslicedInput))
	}
	input := unslicedInput[0:32]

	ct, err := getSingleFheOperand(sess, input)
	if err != nil {
		return err
	}

	operation := FheNot
	err = sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    operation,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      ct,
				FheUintType: handleType(ct),
				IsScalar:    false,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func fheIfThenElseRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 96 {
		return fmt.Errorf("expected at least 96 bytes as input, got %d", len(unslicedInput))
	}
	inputs := unslicedInput[0:96]

	first, second, third, err := getThreeFheOperands(sess, inputs)
	if err != nil {
		return err
	}

	if handleType(second) != handleType(third) {
		return errors.New("fheIfThenElse second argument type doesn't match third argument type")
	}

	operation := FheIfThenElse
	err = sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    operation,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      first,
				FheUintType: handleType(first),
				IsScalar:    false,
			},
			{
				Handle:      second,
				FheUintType: handleType(second),
				IsScalar:    false,
			},
			{
				Handle:      third,
				FheUintType: handleType(third),
				IsScalar:    false,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func castRun(sess ExecutorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 33 {
		return fmt.Errorf("expected at least 33 bytes as input, got %d", len(unslicedInput))
	}

	inputCt := unslicedInput[0:32]
	toType := unslicedInput[32]

	operation := FheCast
	if !IsValidFheType(toType) {
		return fmt.Errorf("invalid fhe type byte: %d", toType)
	}

	sourceCt, err := getSingleFheOperand(sess, inputCt)
	if err != nil {
		return err
	}

	err = sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    operation,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      sourceCt,
				FheUintType: handleType(sourceCt),
				IsScalar:    false,
			},
			{
				Handle:      []byte{toType},
				FheUintType: FheUint8,
				IsScalar:    true,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func fheRandRun(sess ExecutorSession, unslicedInput []byte, ed ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 1 {
		return fmt.Errorf("expected at least 1 bytes as input, got %d", len(unslicedInput))
	}

	resultTypeByte := unslicedInput[0]
	if !IsValidFheType(resultTypeByte) {
		return fmt.Errorf("invalid fhe type byte: %d", resultTypeByte)
	}

	err := sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    FheRand,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      ed.FheRandSeed[:],
				FheUintType: FheUint256,
				IsScalar:    true,
			},
			{
				Handle:      []byte{resultTypeByte},
				FheUintType: FheUint8,
				IsScalar:    true,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func fheRandBoundedRun(sess ExecutorSession, unslicedInput []byte, ed ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 33 {
		return fmt.Errorf("expected at least 1 bytes as input, got %d", len(unslicedInput))
	}

	resultTypeByte := unslicedInput[32]
	if !IsValidFheType(resultTypeByte) {
		return fmt.Errorf("invalid fhe type byte: %d", resultTypeByte)
	}

	upperBound := big.NewInt(0)
	upperBound.SetBytes(unslicedInput[0:32])

	err := sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    FheRandBounded,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      ed.FheRandSeed[:],
				FheUintType: FheUint256,
				IsScalar:    true,
			},
			{
				Handle:      unslicedInput[0:32],
				FheUintType: FheUint256,
				IsScalar:    true,
			},
			{
				Handle:      []byte{resultTypeByte},
				FheUintType: FheUint8,
				IsScalar:    true,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func trivialEncryptRun(sess ExecutorSession, unslicedInput []byte, ed ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 33 {
		return fmt.Errorf("expected at least 1 bytes as input, got %d", len(unslicedInput))
	}

	resultTypeByte := unslicedInput[32]
	if !IsValidFheType(resultTypeByte) {
		return fmt.Errorf("invalid fhe type byte: %d", resultTypeByte)
	}

	err := sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    TrivialEncrypt,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      unslicedInput[0:32],
				FheUintType: FheUint256,
				IsScalar:    true,
			},
			{
				Handle:      []byte{resultTypeByte},
				FheUintType: FheUint8,
				IsScalar:    true,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func trivialEncryptBytesRun(sess ExecutorSession, unslicedInput []byte, ed ExtraData, outputHandle []byte) error {
	if len(unslicedInput) < 96 {
		return fmt.Errorf("expected at least 96 bytes as input, got %d", len(unslicedInput))
	}

	resultTypeByte := unslicedInput[32]

	offsetBigNum := big.NewInt(0)
	offsetBigNum.SetBytes(unslicedInput[0:32])
	startOfByteArray := offsetBigNum.Uint64()
	if startOfByteArray+32 > uint64(len(unslicedInput)) {
		return fmt.Errorf("byte array offset out of bounds, got %d, input length %d", startOfByteArray+32, len(unslicedInput))
	}
	byteArrayLength := big.NewInt(0)
	byteArrayLength.SetBytes(unslicedInput[startOfByteArray : startOfByteArray+32])

	if startOfByteArray+32+byteArrayLength.Uint64() > uint64(len(unslicedInput)) {
		return fmt.Errorf("byte array offset out of bounds, got %d, input length %d", startOfByteArray+32+byteArrayLength.Uint64(), len(unslicedInput))
	}

	// array could be empty
	rawCiphertextByteSlice := []byte{}
	if byteArrayLength.Uint64() > 0 {
		rawCiphertextByteSlice = unslicedInput[startOfByteArray+32 : startOfByteArray+32+byteArrayLength.Uint64()]
	}

	if !IsValidFheType(resultTypeByte) {
		return fmt.Errorf("invalid fhe type byte: %d", resultTypeByte)
	}

	err := sess.GetStore().InsertComputation(ComputationToInsert{
		Operation:    TrivialEncrypt,
		OutputHandle: outputHandle,
		Operands: []ComputationOperand{
			{
				Handle:      rawCiphertextByteSlice,
				FheUintType: FheUint256,
				IsScalar:    true,
			},
			{
				Handle:      []byte{resultTypeByte},
				FheUintType: FheUint8,
				IsScalar:    true,
			},
		},
	})
	if err != nil {
		return err
	}

	return nil
}

func isScalarOp(input []byte) (bool, error) {
	if len(input) != 65 {
		return false, errors.New("input needs to contain two 256-bit sized values and 1 8-bit value")
	}
	isScalar := (input[64] == 1)
	return isScalar, nil
}

func get2FheOperands(sess ExecutorSession, input []byte) (lhs []byte, rhs []byte, err error) {
	if len(input) != 65 {
		return nil, nil, errors.New("input needs to contain two 256-bit sized values and 1 8-bit value")
	}
	return input[0:32], input[32:64], nil
}

func getSingleFheOperand(sess ExecutorSession, input []byte) (operand []byte, err error) {
	if len(input) != 32 {
		return nil, errors.New("input needs to contain one 256-bit sized value")
	}
	return input[0:32], nil
}

func getScalarOperands(sess ExecutorSession, input []byte) (lhs []byte, rhs []byte, err error) {
	if len(input) != 65 {
		return nil, nil, errors.New("input needs to contain two 256-bit sized values and 1 8-bit value")
	}
	return input[0:32], input[32:64], nil
}

func getThreeFheOperands(sess ExecutorSession, input []byte) (first []byte, second []byte, third []byte, err error) {
	if len(input) != 96 {
		return nil, nil, nil, errors.New("input needs to contain three 256-bit sized values")
	}

	return input[0:32], input[32:64], input[64:96], nil
}

func isBinaryOp(op FheOp) bool {
	switch op {
	case FheAdd, FheBitAnd, FheBitOr, FheBitXor, FheDiv, FheEq, FheGe, FheGt, FheLe, FheLt, FheMax, FheMin, FheMul, FheNe, FheRem, FheRotl, FheRotr, FheShl, FheShr, FheSub:
		return true
	case FheCast, FheNeg, FheNot, FheRand, FheRandBounded, FheIfThenElse, TrivialEncrypt:
		return false
	default:
		return false
	}
}

func isUnaryOp(op FheOp) bool {
	switch op {
	case FheNeg, FheNot:
		return true
	case FheAdd, FheBitAnd, FheBitOr, FheBitXor, FheDiv, FheEq, FheGe, FheGt, FheLe, FheLt, FheMax, FheMin, FheMul, FheNe, FheRem, FheRotl, FheRotr, FheShl, FheShr, FheSub, FheCast, FheRand, FheRandBounded, FheIfThenElse, TrivialEncrypt:
		return false
	default:
		return false
	}
}
