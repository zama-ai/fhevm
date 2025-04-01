import { Operation } from '../repositories/dapp.repository.js'

// Initialize all operations with 0
const allOperations: Record<Operation, number> = {
  FheAdd: 0,
  FheBitAnd: 0,
  FheIfThenElse: 0,
  FheLe: 0,
  FheOr: 0,
  FheSub: 0,
  TrivialEncrypt: 0,
  VerifyCiphertext: 0,
  FheMul: 0,
  FheDiv: 0,
}

export class Computation {
  private readonly values: Record<Operation, number>

  constructor(values: Partial<Record<Operation, number>> = {}) {
    this.values = { ...allOperations, ...values }
  }

  get FheAdd() {
    return this.values['FheAdd']
  }

  get FheBitAnd() {
    return this.values['FheBitAnd']
  }

  get FheIfThenElse() {
    return this.values['FheIfThenElse']
  }

  get FheLe() {
    return this.values['FheLe']
  }

  get FheOr() {
    return this.values['FheOr']
  }

  get FheSub() {
    return this.values['FheSub']
  }

  get TrivialEncrypt() {
    return this.values['TrivialEncrypt']
  }

  get VerifyCiphertext() {
    return this.values['VerifyCiphertext']
  }

  get FheMul() {
    return this.values['FheMul']
  }

  get FheDiv() {
    return this.values['FheDiv']
  }

  get total() {
    return Object.values(this.values).reduce((acc, i) => acc + i)
  }

  toJSON() {
    return {
      ...this.values,
      total: this.total,
    }
  }
}
