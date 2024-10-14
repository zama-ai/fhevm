package fhevm

import (
	"errors"
	"fmt"
	"math/big"
)

func handleType(handle []byte) FheUintType {
	return FheUintType(handle[30])
}

func fheAddRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheSubRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheMulRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheRemRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheBitAndRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheBitOrRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheBitXorRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheShlRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheShrRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheRotlRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheRotrRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheEqRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheNeRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheGeRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheGtRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheLeRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheLtRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheMinRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheMaxRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheNegRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheNotRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheIfThenElseRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func castRun(sess CoprocessorSession, unslicedInput []byte, _ ExtraData, outputHandle []byte) error {
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

func fheRandRun(sess CoprocessorSession, unslicedInput []byte, ed ExtraData, outputHandle []byte) error {
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

func fheRandBoundedRun(sess CoprocessorSession, unslicedInput []byte, ed ExtraData, outputHandle []byte) error {
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

func trivialEncryptRun(sess CoprocessorSession, unslicedInput []byte, ed ExtraData, outputHandle []byte) error {
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

func isScalarOp(input []byte) (bool, error) {
	if len(input) != 65 {
		return false, errors.New("input needs to contain two 256-bit sized values and 1 8-bit value")
	}
	isScalar := (input[64] == 1)
	return isScalar, nil
}

func get2FheOperands(sess CoprocessorSession, input []byte) (lhs []byte, rhs []byte, err error) {
	if len(input) != 65 {
		return nil, nil, errors.New("input needs to contain two 256-bit sized values and 1 8-bit value")
	}
	return input[0:32], input[32:64], nil
}

func getSingleFheOperand(sess CoprocessorSession, input []byte) (operand []byte, err error) {
	if len(input) != 32 {
		return nil, errors.New("input needs to contain one 256-bit sized value")
	}
	return input[0:32], nil
}

func getScalarOperands(sess CoprocessorSession, input []byte) (lhs []byte, rhs []byte, err error) {
	if len(input) != 65 {
		return nil, nil, errors.New("input needs to contain two 256-bit sized values and 1 8-bit value")
	}
	return input[0:32], input[32:64], nil
}

func getThreeFheOperands(sess CoprocessorSession, input []byte) (first []byte, second []byte, third []byte, err error) {
	if len(input) != 96 {
		return nil, nil, nil, errors.New("input needs to contain three 256-bit sized values")
	}

	return input[0:32], input[32:64], input[64:96], nil
}
