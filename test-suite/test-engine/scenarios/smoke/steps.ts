import { type DataTable, Given, Then, When, World, setWorldConstructor } from '@cucumber/cucumber';
import assert from 'node:assert/strict';

/** Scenario state. A fresh instance is created for every Gherkin scenario. */
class ArithmeticWorld extends World {
  numbers: number[] = [];
  result: number | undefined;
}

setWorldConstructor(ArithmeticWorld);

function parseNumbers(values: string[]): number[] {
  const numbers = values.map((value) => Number(value.trim()));
  assert.ok(numbers.every(Number.isFinite), `every value must be a finite number: ${values.join(', ')}`);
  return numbers;
}

Given('the number {int}', function (this: ArithmeticWorld, value: number) {
  this.numbers.push(value);
});

Given('the numbers:', function (this: ArithmeticWorld, table: DataTable) {
  this.numbers.push(...parseNumbers(table.hashes().map((row) => row.value ?? '')));
});

Given('the newline-separated numbers:', function (this: ArithmeticWorld, docString: string) {
  this.numbers.push(...parseNumbers(docString.split(/\r?\n/).filter((line) => line.trim() !== '')));
});

When('the numbers are added', function (this: ArithmeticWorld) {
  this.result = this.numbers.reduce((sum, value) => sum + value, 0);
});

Then('the result is {int}', function (this: ArithmeticWorld, expected: number) {
  assert.equal(this.result, expected);
});
