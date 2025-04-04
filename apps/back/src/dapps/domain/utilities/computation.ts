import { Operation } from '../repositories/dapp.repository.js'

// Initialize all operations with 0
const allOperations: Record<Operation, number> = {
  FheAdd: 0,
  FheSub: 0,
  FheMul: 0,
  FheDiv: 0,
  FheRem: 0,
  FheBitAnd: 0,
  FheBitOr: 0,
  FheBitXor: 0,
  FheShl: 0,
  FheShr: 0,
  FheRotl: 0,
  FheRotr: 0,
  FheEq: 0,
  FheEqBytes: 0,
  FheNe: 0,
  FheNeBytes: 0,
  FheGe: 0,
  FheGt: 0,
  FheLe: 0,
  FheLt: 0,
  FheMin: 0,
  FheMax: 0,
  FheNeg: 0,
  FheNot: 0,
  VerifyCiphertext: 0,
  Cast: 0,
  TrivialEncrypt: 0,
  TrivialEncryptBytes: 0,
  FheIfThenElse: 0,
  FheRand: 0,
  FheRandBounded: 0,
}

export class Computation {
  private readonly values: Record<Operation, number>

  constructor(values: Partial<Record<Operation, number>> = {}) {
    this.values = { ...allOperations, ...values }
  }

  get FheAdd() {
    return this.values['FheAdd']
  }

  get FheSub() {
    return this.values['FheSub']
  }

  get FheMul() {
    return this.values['FheMul']
  }

  get FheDiv() {
    return this.values['FheDiv']
  }

  get FheRem() {
    return this.values['FheRem']
  }

  get FheBitAnd() {
    return this.values['FheBitAnd']
  }

  get FheBitOr() {
    return this.values['FheBitOr']
  }

  get FheBitXor() {
    return this.values['FheBitXor']
  }

  get FheShl() {
    return this.values['FheShl']
  }

  get FheShr() {
    return this.values['FheShr']
  }

  get FheRotl() {
    return this.values['FheRotl']
  }

  get FheRotr() {
    return this.values['FheRotr']
  }

  get FheEq() {
    return this.values['FheEq']
  }

  get FheEqBytes() {
    return this.values['FheEqBytes']
  }

  get FheNe() {
    return this.values['FheNe']
  }

  get FheNeBytes() {
    return this.values['FheNeBytes']
  }

  get FheGe() {
    return this.values['FheGe']
  }

  get FheGt() {
    return this.values['FheGt']
  }

  get FheLe() {
    return this.values['FheLe']
  }

  get FheLt() {
    return this.values['FheLt']
  }

  get FheMin() {
    return this.values['FheMin']
  }

  get FheMax() {
    return this.values['FheMax']
  }

  get FheNeg() {
    return this.values['FheNeg']
  }

  get FheNot() {
    return this.values['FheNot']
  }

  get VerifyCiphertext() {
    return this.values['VerifyCiphertext']
  }

  get Cast() {
    return this.values['Cast']
  }

  get TrivialEncrypt() {
    return this.values['TrivialEncrypt']
  }

  get TrivialEncryptBytes() {
    return this.values['TrivialEncryptBytes']
  }

  get FheIfThenElse() {
    return this.values['FheIfThenElse']
  }

  get FheRand() {
    return this.values['FheRand']
  }

  get FheRandBounded() {
    return this.values['FheRandBounded']
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
