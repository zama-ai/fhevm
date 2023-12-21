import { OverloadSignature, signatureContractMethodName } from './testgen';

type OverloadTest = {
  inputs: number[];
  output: boolean | number;
};

export const overloadTests: { [methodName: string]: OverloadTest[] } = {
  // solidity operators 8 bit
  bin_op_add_euint8_euint8: [{ inputs: [0x03, 0x04], output: 0x07 }],
  bin_op_sub_euint8_euint8: [{ inputs: [0x04, 0x03], output: 0x01 }],
  bin_op_mul_euint8_euint8: [{ inputs: [0x04, 0x03], output: 12 }],
  bin_op_and_euint8_euint8: [{ inputs: [0xef, 0xf0], output: 0xe0 }],
  bin_op_or_euint8_euint8: [{ inputs: [0xef, 0xf0], output: 0xff }],
  bin_op_xor_euint8_euint8: [{ inputs: [0xef, 0xf0], output: 0x1f }],
  unary_op_neg_euint8: [{ inputs: [0x02], output: 0xfe }],
  unary_op_not_euint8: [{ inputs: [0x0f], output: 0xf0 }],

  // solidity operators 16 bit
  bin_op_add_euint16_euint16: [{ inputs: [0x0103, 0x0204], output: 0x0307 }],
  bin_op_sub_euint16_euint16: [{ inputs: [0x0204, 0x0103], output: 0x0101 }],
  bin_op_mul_euint16_euint16: [{ inputs: [0x0104, 0x0003], output: 0x030c }],
  bin_op_and_euint16_euint16: [{ inputs: [0xefef, 0xf0f0], output: 0xe0e0 }],
  bin_op_or_euint16_euint16: [{ inputs: [0xefef, 0x01f0], output: 0xefff }],
  bin_op_xor_euint16_euint16: [{ inputs: [0xefef, 0xf0f0], output: 0x1f1f }],
  unary_op_neg_euint16: [{ inputs: [0x0003], output: 0xfffd }],
  unary_op_not_euint16: [{ inputs: [0x0f0f], output: 0xf0f0 }],

  // solidity operators 32 bit
  bin_op_add_euint32_euint32: [{ inputs: [0x00100103, 0x00400204], output: 0x00500307 }],
  bin_op_sub_euint32_euint32: [{ inputs: [0x90000204, 0x50000103], output: 0x40000101 }],
  bin_op_mul_euint32_euint32: [{ inputs: [0x02000104, 0x0003], output: 0x0600030c }],
  bin_op_and_euint32_euint32: [{ inputs: [0xefefefef, 0xf0f0f0f0], output: 0xe0e0e0e0 }],
  bin_op_or_euint32_euint32: [{ inputs: [0xefefefef, 0x01f001f0], output: 0xefffefff }],
  bin_op_xor_euint32_euint32: [{ inputs: [0xefefefef, 0xf0f0f0f0], output: 0x1f1f1f1f }],
  unary_op_neg_euint32: [{ inputs: [0x00000004], output: 0xfffffffc }],
  unary_op_not_euint32: [{ inputs: [0x0f0f0f0f], output: 0xf0f0f0f0 }],

  neg_euint8: [
    { inputs: [0x01], output: 0xff },
    { inputs: [0x02], output: 0xfe },
  ],
  not_euint8: [{ inputs: [0x03], output: 0xfc }],
  neg_euint16: [
    { inputs: [0x0001], output: 0xffff },
    { inputs: [0x0002], output: 0xfffe },
  ],
  not_euint16: [{ inputs: [0x00f1], output: 0xff0e }],
  neg_euint32: [
    { inputs: [0x00000001], output: 0xffffffff },
    { inputs: [0x00000002], output: 0xfffffffe },
  ],
  not_euint32: [{ inputs: [0x0000fffe], output: 0xffff0001 }],
  add_euint8_euint8: [{ inputs: [3, 4], output: 7 }],
  sub_euint8_euint8: [{ inputs: [4, 3], output: 1 }],
  mul_euint8_euint8: [{ inputs: [3, 4], output: 12 }],
  and_euint8_euint8: [{ inputs: [0xff, 0x0f], output: 0x0f }],
  or_euint8_euint8: [{ inputs: [0x70, 0x0f], output: 0x7f }],
  xor_euint8_euint8: [
    { inputs: [0x77, 0x77], output: 0x00 },
    { inputs: [12, 34], output: 46 },
  ],
  shl_euint8_euint8: [
    { inputs: [2, 1], output: 4 },
    { inputs: [2, 4], output: 32 },
  ],
  shr_euint8_euint8: [
    { inputs: [2, 1], output: 1 },
    { inputs: [32, 4], output: 2 },
  ],
  eq_euint8_euint8: [
    { inputs: [12, 49], output: false },
    { inputs: [7, 7], output: true },
  ],
  ne_euint8_euint8: [
    { inputs: [1, 2], output: true },
    { inputs: [2, 2], output: false },
  ],
  ge_euint8_euint8: [
    { inputs: [10, 10], output: true },
    { inputs: [10, 9], output: true },
    { inputs: [10, 11], output: false },
  ],
  gt_euint8_euint8: [
    { inputs: [10, 10], output: false },
    { inputs: [10, 9], output: true },
    { inputs: [10, 11], output: false },
  ],
  le_euint8_euint8: [
    { inputs: [10, 10], output: true },
    { inputs: [10, 9], output: false },
    { inputs: [10, 11], output: true },
  ],
  lt_euint8_euint8: [
    { inputs: [10, 10], output: false },
    { inputs: [10, 9], output: false },
    { inputs: [10, 11], output: true },
  ],
  min_euint8_euint8: [
    { inputs: [10, 10], output: 10 },
    { inputs: [12, 10], output: 10 },
    { inputs: [9, 12], output: 9 },
  ],
  max_euint8_euint8: [
    { inputs: [10, 10], output: 10 },
    { inputs: [12, 10], output: 12 },
    { inputs: [9, 12], output: 12 },
  ],
  add_euint8_euint16: [{ inputs: [0x03, 0xff00], output: 0xff03 }],
  sub_euint8_euint16: [{ inputs: [0x03, 0x1000], output: 0xf003 }],
  mul_euint8_euint16: [{ inputs: [0x03, 0x1000], output: 0x3000 }],
  and_euint8_euint16: [
    { inputs: [0x03, 0x1000], output: 0x0000 },
    { inputs: [0x03, 0x1001], output: 0x0001 },
  ],
  or_euint8_euint16: [
    { inputs: [0x03, 0x1000], output: 0x1003 },
    { inputs: [0x03, 0x1001], output: 0x1003 },
  ],
  xor_euint8_euint16: [
    { inputs: [0xff, 0xffff], output: 0xff00 },
    { inputs: [0xff, 0xff00], output: 0xffff },
  ],
  eq_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: true },
    { inputs: [0xff, 0x01ff], output: false },
  ],
  ne_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: false },
    { inputs: [0xff, 0x01ff], output: true },
  ],
  ge_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: true },
    { inputs: [0xff, 0x01ff], output: false },
    { inputs: [0xff, 0x007f], output: true },
  ],
  gt_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: false },
    { inputs: [0xff, 0x01ff], output: false },
    { inputs: [0xff, 0x007f], output: true },
  ],
  le_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: true },
    { inputs: [0xff, 0x01ff], output: true },
    { inputs: [0xff, 0x007f], output: false },
  ],
  lt_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: false },
    { inputs: [0xff, 0x01ff], output: true },
    { inputs: [0xff, 0x007f], output: false },
  ],
  min_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: 0xff },
    { inputs: [0xff, 0x01ff], output: 0xff },
    { inputs: [0xff, 0x007f], output: 0x7f },
  ],
  max_euint8_euint16: [
    { inputs: [0xff, 0x00ff], output: 0xff },
    { inputs: [0xff, 0x01ff], output: 0x1ff },
    { inputs: [0xff, 0x007f], output: 0xff },
  ],
  add_euint8_euint32: [{ inputs: [0xff, 0xffff00ff], output: 0xffff01fe }],
  sub_euint8_euint32: [
    { inputs: [0xff, 0xffff00ff], output: 0x10000 },
    { inputs: [0xff, 0x00000010], output: 0x000ef },
  ],
  mul_euint8_euint32: [{ inputs: [0x10, 0x00010000], output: 0x00100000 }],
  and_euint8_euint32: [
    { inputs: [0x10, 0x00010000], output: 0x00000000 },
    { inputs: [0x11, 0x00010010], output: 0x00000010 },
  ],
  or_euint8_euint32: [
    { inputs: [0x10, 0x00010000], output: 0x00010010 },
    { inputs: [0x11, 0x00010010], output: 0x00010011 },
  ],
  xor_euint8_euint32: [
    { inputs: [0x10, 0x00010000], output: 0x00010010 },
    { inputs: [0x11, 0x00010010], output: 0x00010001 },
  ],
  eq_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: true },
    { inputs: [0x01, 0x00010001], output: false },
  ],
  ne_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: false },
    { inputs: [0x01, 0x00010001], output: true },
  ],
  ge_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: true },
    { inputs: [0x01, 0x00010001], output: false },
    { inputs: [0x10, 0x00000001], output: true },
  ],
  gt_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: false },
    { inputs: [0x01, 0x00010001], output: false },
    { inputs: [0x10, 0x00000001], output: true },
  ],
  le_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: true },
    { inputs: [0x01, 0x00010001], output: true },
    { inputs: [0x10, 0x00000001], output: false },
  ],
  lt_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: false },
    { inputs: [0x01, 0x00010001], output: true },
    { inputs: [0x10, 0x00000001], output: false },
  ],
  min_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: 0x1 },
    { inputs: [0x01, 0x00010001], output: 0x1 },
    { inputs: [0x10, 0x00000004], output: 0x4 },
  ],
  max_euint8_euint32: [
    { inputs: [0x01, 0x00000001], output: 0x1 },
    { inputs: [0x01, 0x00010001], output: 0x10001 },
    { inputs: [0x10, 0x00000004], output: 0x10 },
  ],
  add_euint8_uint8: [{ inputs: [0x04, 0x03], output: 0x07 }],
  add_uint8_euint8: [{ inputs: [0x04, 0x03], output: 0x07 }],
  sub_euint8_uint8: [
    // bug found
    { inputs: [0x04, 0x03], output: 0x01 },
    { inputs: [0x03, 0x04], output: 0xff },
  ],
  sub_uint8_euint8: [
    // bug found
    { inputs: [0x04, 0x03], output: 0x01 },
    { inputs: [0x03, 0x04], output: 0xff },
  ],
  mul_euint8_uint8: [
    { inputs: [0x04, 0x03], output: 0x0c },
    { inputs: [0x03, 0x04], output: 0x0c },
    { inputs: [0x08, 0x02], output: 0x10 },
  ],
  mul_uint8_euint8: [
    { inputs: [0x04, 0x03], output: 0x0c },
    { inputs: [0x03, 0x04], output: 0x0c },
    { inputs: [0x08, 0x02], output: 0x10 },
  ],
  div_euint8_uint8: [{ inputs: [0x10, 0x02], output: 0x08 }],
  rem_euint8_uint8: [{ inputs: [0x08, 0x03], output: 0x02 }],
  shl_euint8_uint8: [
    { inputs: [0x10, 0x01], output: 0x20 },
    { inputs: [0x10, 0x02], output: 0x40 },
  ],
  shr_euint8_uint8: [
    { inputs: [0x10, 0x01], output: 0x08 },
    { inputs: [0x10, 0x02], output: 0x04 },
  ],
  eq_euint8_uint8: [
    { inputs: [0x10, 0x10], output: true },
    { inputs: [0x10, 0x02], output: false },
  ],
  eq_uint8_euint8: [
    { inputs: [0x10, 0x10], output: true },
    { inputs: [0x10, 0x02], output: false },
  ],
  ne_euint8_uint8: [
    { inputs: [0x10, 0x10], output: false },
    { inputs: [0x10, 0x02], output: true },
  ],
  ne_uint8_euint8: [
    { inputs: [0x10, 0x10], output: false },
    { inputs: [0x10, 0x02], output: true },
  ],
  ge_euint8_uint8: [
    { inputs: [0x10, 0x10], output: true },
    { inputs: [0x10, 0x02], output: true },
    { inputs: [0x10, 0x11], output: false },
  ],
  ge_uint8_euint8: [
    { inputs: [0x10, 0x10], output: true },
    { inputs: [0x10, 0x02], output: true },
    { inputs: [0x10, 0x11], output: false },
  ],
  gt_euint8_uint8: [
    { inputs: [0x10, 0x10], output: false },
    { inputs: [0x10, 0x02], output: true },
    { inputs: [0x10, 0x11], output: false },
  ],
  gt_uint8_euint8: [
    { inputs: [0x10, 0x10], output: false },
    { inputs: [0x10, 0x02], output: true },
    { inputs: [0x10, 0x11], output: false },
  ],
  le_euint8_uint8: [
    { inputs: [0x10, 0x10], output: true },
    { inputs: [0x10, 0x02], output: false },
    { inputs: [0x10, 0x11], output: true },
  ],
  le_uint8_euint8: [
    { inputs: [0x10, 0x10], output: true },
    { inputs: [0x10, 0x02], output: false },
    { inputs: [0x10, 0x11], output: true },
  ],
  lt_euint8_uint8: [
    { inputs: [0x10, 0x10], output: false },
    { inputs: [0x10, 0x02], output: false },
    { inputs: [0x10, 0x11], output: true },
  ],
  lt_uint8_euint8: [
    { inputs: [0x10, 0x10], output: false },
    { inputs: [0x10, 0x02], output: false },
    { inputs: [0x10, 0x11], output: true },
  ],
  min_euint8_uint8: [
    { inputs: [0x10, 0x10], output: 0x10 },
    { inputs: [0x10, 0x02], output: 0x02 },
    { inputs: [0x10, 0x11], output: 0x10 },
  ],
  min_uint8_euint8: [
    { inputs: [0x10, 0x10], output: 0x10 },
    { inputs: [0x10, 0x02], output: 0x02 },
    { inputs: [0x10, 0x11], output: 0x10 },
  ],
  max_euint8_uint8: [
    { inputs: [0x10, 0x10], output: 0x10 },
    { inputs: [0x10, 0x02], output: 0x10 },
    { inputs: [0x10, 0x11], output: 0x11 },
  ],
  max_uint8_euint8: [
    { inputs: [0x10, 0x10], output: 0x10 },
    { inputs: [0x10, 0x02], output: 0x10 },
    { inputs: [0x10, 0x11], output: 0x11 },
  ],
  add_euint16_euint8: [
    { inputs: [0x1000, 0x10], output: 0x1010 },
    { inputs: [0x1010, 0x10], output: 0x1020 },
  ],
  sub_euint16_euint8: [
    { inputs: [0x1000, 0x10], output: 0x0ff0 },
    { inputs: [0x1010, 0x10], output: 0x1000 },
  ],
  mul_euint16_euint8: [{ inputs: [0x1000, 0x04], output: 0x4000 }],
  and_euint16_euint8: [
    { inputs: [0x1000, 0x04], output: 0x0000 },
    { inputs: [0x10f0, 0xf0], output: 0x00f0 },
  ],
  or_euint16_euint8: [
    { inputs: [0x1000, 0x04], output: 0x1004 },
    { inputs: [0x10f0, 0xf0], output: 0x10f0 },
  ],
  xor_euint16_euint8: [
    { inputs: [0x1000, 0x04], output: 0x1004 },
    { inputs: [0x10f0, 0xf2], output: 0x1002 },
  ],
  shl_euint16_euint8: [{ inputs: [0x1010, 0x02], output: 0x4040 }],
  shl_euint16_uint8: [{ inputs: [0x1010, 0x02], output: 0x4040 }],
  shr_euint16_euint8: [{ inputs: [0x1010, 0x02], output: 0x0404 }],
  shr_euint16_uint8: [{ inputs: [0x1010, 0x02], output: 0x0404 }],
  eq_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: true },
    { inputs: [0x0110, 0x10], output: false },
  ],
  ne_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: false },
    { inputs: [0x0110, 0x10], output: true },
  ],
  ge_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: true },
    { inputs: [0x0110, 0x10], output: true },
    { inputs: [0x000f, 0x10], output: false },
  ],
  gt_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: false },
    { inputs: [0x0110, 0x10], output: true },
    { inputs: [0x000f, 0x10], output: false },
  ],
  le_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: true },
    { inputs: [0x0110, 0x10], output: false },
    { inputs: [0x000f, 0x10], output: true },
  ],
  lt_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: false },
    { inputs: [0x0110, 0x10], output: false },
    { inputs: [0x000f, 0x10], output: true },
  ],
  min_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: 0x10 },
    { inputs: [0x0110, 0x10], output: 0x10 },
    { inputs: [0x000f, 0x10], output: 0x0f },
  ],
  max_euint16_euint8: [
    { inputs: [0x0010, 0x10], output: 0x10 },
    { inputs: [0x0110, 0x10], output: 0x0110 },
    { inputs: [0x000f, 0x10], output: 0x10 },
  ],
  add_euint16_euint16: [{ inputs: [0x0102, 0x0201], output: 0x0303 }],
  sub_euint16_euint16: [{ inputs: [0x0403, 0x0102], output: 0x0301 }],
  mul_euint16_euint16: [{ inputs: [0x0200, 0x0002], output: 0x0400 }],
  and_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: 0x0000 },
    { inputs: [0x0210, 0x0012], output: 0x0010 },
  ],
  or_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: 0x0202 },
    { inputs: [0x0210, 0x0012], output: 0x0212 },
  ],
  xor_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: 0x0202 },
    { inputs: [0x0210, 0x0012], output: 0x0202 },
  ],
  eq_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: false },
    { inputs: [0x0200, 0x0200], output: true },
  ],
  ne_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: true },
    { inputs: [0x0200, 0x0200], output: false },
  ],
  ge_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: true },
    { inputs: [0x0200, 0x0200], output: true },
    { inputs: [0x0200, 0x0201], output: false },
  ],
  gt_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: true },
    { inputs: [0x0200, 0x0200], output: false },
    { inputs: [0x0200, 0x0201], output: false },
  ],
  le_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: false },
    { inputs: [0x0200, 0x0200], output: true },
    { inputs: [0x0200, 0x0201], output: true },
  ],
  lt_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: false },
    { inputs: [0x0200, 0x0200], output: false },
    { inputs: [0x0200, 0x0201], output: true },
  ],
  min_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: 0x02 },
    { inputs: [0x0200, 0x0200], output: 0x0200 },
    { inputs: [0x0200, 0x0201], output: 0x0200 },
  ],
  max_euint16_euint16: [
    { inputs: [0x0200, 0x0002], output: 0x0200 },
    { inputs: [0x0200, 0x0200], output: 0x0200 },
    { inputs: [0x0200, 0x0201], output: 0x0201 },
  ],
  add_euint16_euint32: [{ inputs: [0x0202, 0x00020002], output: 0x00020204 }],
  sub_euint16_euint32: [
    { inputs: [0x0202, 0x00000002], output: 0x00000200 },
    { inputs: [0x0202, 0x00010000], output: 0xffff0202 },
  ],
  mul_euint16_euint32: [{ inputs: [0x0200, 0x00010000], output: 0x02000000 }],
  and_euint16_euint32: [
    { inputs: [0x0202, 0x00010000], output: 0x00000000 },
    { inputs: [0x0202, 0x00010002], output: 0x00000002 },
  ],
  or_euint16_euint32: [
    { inputs: [0x0202, 0x00010000], output: 0x00010202 },
    { inputs: [0x0202, 0x00010002], output: 0x00010202 },
  ],
  xor_euint16_euint32: [
    { inputs: [0x0202, 0x00010000], output: 0x00010202 },
    { inputs: [0x0202, 0x00010002], output: 0x00010200 },
  ],
  eq_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: false },
    { inputs: [0x0202, 0x00000202], output: true },
  ],
  ne_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: true },
    { inputs: [0x0202, 0x00000202], output: false },
  ],
  ge_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: false },
    { inputs: [0x0202, 0x00000202], output: true },
    { inputs: [0x0202, 0x00000201], output: true },
  ],
  gt_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: false },
    { inputs: [0x0202, 0x00000202], output: false },
    { inputs: [0x0202, 0x00000201], output: true },
  ],
  le_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: true },
    { inputs: [0x0202, 0x00000202], output: true },
    { inputs: [0x0202, 0x00000201], output: false },
  ],
  lt_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: true },
    { inputs: [0x0202, 0x00000202], output: false },
    { inputs: [0x0202, 0x00000201], output: false },
  ],
  min_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: 0x202 },
    { inputs: [0x0202, 0x00000202], output: 0x202 },
    { inputs: [0x0202, 0x00000201], output: 0x201 },
  ],
  max_euint16_euint32: [
    { inputs: [0x0202, 0x00010202], output: 0x00010202 },
    { inputs: [0x0202, 0x00000202], output: 0x202 },
    { inputs: [0x0202, 0x00000201], output: 0x202 },
  ],
  add_euint16_uint16: [{ inputs: [0x0202, 0x0222], output: 0x0424 }],
  add_uint16_euint16: [{ inputs: [0x0202, 0x0222], output: 0x0424 }],
  sub_euint16_uint16: [{ inputs: [0x0202, 0x0201], output: 0x0001 }],
  sub_uint16_euint16: [{ inputs: [0x0202, 0x0201], output: 0x0001 }],
  mul_euint16_uint16: [{ inputs: [0x0202, 0x0003], output: 0x0606 }],
  mul_uint16_euint16: [{ inputs: [0x0202, 0x0003], output: 0x0606 }],
  div_euint16_uint16: [{ inputs: [0x0606, 0x0003], output: 0x0202 }],
  rem_euint16_uint16: [{ inputs: [0x0608, 0x0003], output: 0x0002 }],
  eq_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: true },
    { inputs: [0x0606, 0x0605], output: false },
  ],
  eq_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: true },
    { inputs: [0x0606, 0x0605], output: false },
  ],
  ne_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: false },
    { inputs: [0x0606, 0x0605], output: true },
  ],
  ne_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: false },
    { inputs: [0x0606, 0x0605], output: true },
  ],
  ge_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: true },
    { inputs: [0x0606, 0x0605], output: true },
    { inputs: [0x0606, 0x0607], output: false },
  ],
  ge_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: true },
    { inputs: [0x0606, 0x0605], output: true },
    { inputs: [0x0606, 0x0607], output: false },
  ],
  gt_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: false },
    { inputs: [0x0606, 0x0605], output: true },
    { inputs: [0x0606, 0x0607], output: false },
  ],
  gt_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: false },
    { inputs: [0x0606, 0x0605], output: true },
    { inputs: [0x0606, 0x0607], output: false },
  ],
  le_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: true },
    { inputs: [0x0606, 0x0605], output: false },
    { inputs: [0x0606, 0x0607], output: true },
  ],
  le_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: true },
    { inputs: [0x0606, 0x0605], output: false },
    { inputs: [0x0606, 0x0607], output: true },
  ],
  lt_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: false },
    { inputs: [0x0606, 0x0605], output: false },
    { inputs: [0x0606, 0x0607], output: true },
  ],
  lt_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: false },
    { inputs: [0x0606, 0x0605], output: false },
    { inputs: [0x0606, 0x0607], output: true },
  ],
  min_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: 0x0606 },
    { inputs: [0x0606, 0x0605], output: 0x0605 },
    { inputs: [0x0606, 0x0607], output: 0x0606 },
  ],
  min_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: 0x0606 },
    { inputs: [0x0606, 0x0605], output: 0x0605 },
    { inputs: [0x0606, 0x0607], output: 0x0606 },
  ],
  max_euint16_uint16: [
    { inputs: [0x0606, 0x0606], output: 0x0606 },
    { inputs: [0x0606, 0x0605], output: 0x0606 },
    { inputs: [0x0606, 0x0607], output: 0x0607 },
  ],
  max_uint16_euint16: [
    { inputs: [0x0606, 0x0606], output: 0x0606 },
    { inputs: [0x0606, 0x0605], output: 0x0606 },
    { inputs: [0x0606, 0x0607], output: 0x0607 },
  ],
  add_euint32_euint8: [{ inputs: [0x03000000, 0x03], output: 0x03000003 }],
  sub_euint32_euint8: [{ inputs: [0x03000000, 0x03], output: 0x2fffffd }],
  mul_euint32_euint8: [{ inputs: [0x03000000, 0x03], output: 0x09000000 }],
  and_euint32_euint8: [
    { inputs: [0x03010000, 0x03], output: 0x00000000 },
    { inputs: [0x03010003, 0x03], output: 0x00000003 },
  ],
  or_euint32_euint8: [
    { inputs: [0x03010000, 0x03], output: 0x03010003 },
    { inputs: [0x03010003, 0x03], output: 0x03010003 },
  ],
  xor_euint32_euint8: [
    { inputs: [0x03010000, 0x03], output: 0x03010003 },
    { inputs: [0x03010003, 0x03], output: 0x03010000 },
  ],
  shl_euint32_euint8: [{ inputs: [0x03010000, 0x03], output: 0x18080000 }],
  shl_euint32_uint8: [{ inputs: [0x03010000, 0x03], output: 0x18080000 }],
  shr_euint32_euint8: [{ inputs: [0x03010000, 0x03], output: 0x00602000 }],
  shr_euint32_uint8: [{ inputs: [0x03010000, 0x03], output: 0x00602000 }],
  eq_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: true },
    { inputs: [0x03000003, 0x03], output: false },
  ],
  ne_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: false },
    { inputs: [0x03000003, 0x03], output: true },
  ],
  ge_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: true },
    { inputs: [0x03000003, 0x03], output: true },
    { inputs: [0x00000003, 0x04], output: false },
  ],
  gt_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: false },
    { inputs: [0x03000003, 0x03], output: true },
    { inputs: [0x00000003, 0x04], output: false },
  ],
  le_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: true },
    { inputs: [0x03000003, 0x03], output: false },
    { inputs: [0x00000003, 0x04], output: true },
  ],
  lt_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: false },
    { inputs: [0x03000003, 0x03], output: false },
    { inputs: [0x00000003, 0x04], output: true },
  ],
  min_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: 0x03 },
    { inputs: [0x03000003, 0x03], output: 0x03 },
    { inputs: [0x00000003, 0x04], output: 0x03 },
  ],
  max_euint32_euint8: [
    { inputs: [0x00000003, 0x03], output: 0x03 },
    { inputs: [0x03000003, 0x03], output: 0x03000003 },
    { inputs: [0x00000003, 0x04], output: 0x04 },
  ],
  add_euint32_euint16: [{ inputs: [0x03001023, 0x1003], output: 0x03002026 }],
  sub_euint32_euint16: [{ inputs: [0x03001023, 0x1003], output: 0x03000020 }],
  mul_euint32_euint16: [{ inputs: [0x03001023, 0x0003], output: 0x09003069 }],
  and_euint32_euint16: [
    { inputs: [0x03001020, 0x0003], output: 0x00000000 },
    { inputs: [0x03001023, 0x1003], output: 0x00001003 },
  ],
  or_euint32_euint16: [
    { inputs: [0x03000020, 0x1003], output: 0x03001023 },
    { inputs: [0x03000023, 0x1003], output: 0x03001023 },
  ],
  xor_euint32_euint16: [{ inputs: [0x03000023, 0x1003], output: 0x03001020 }],
  eq_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: true },
    { inputs: [0x01001000, 0x1000], output: false },
  ],
  ne_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: false },
    { inputs: [0x01001000, 0x1000], output: true },
  ],
  ge_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: true },
    { inputs: [0x01001000, 0x1000], output: true },
    { inputs: [0x00001000, 0x1001], output: false },
  ],
  gt_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: false },
    { inputs: [0x01001000, 0x1000], output: true },
    { inputs: [0x00001000, 0x1001], output: false },
  ],
  le_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: true },
    { inputs: [0x01001000, 0x1000], output: false },
    { inputs: [0x00001000, 0x1001], output: true },
  ],
  lt_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: false },
    { inputs: [0x01001000, 0x1000], output: false },
    { inputs: [0x00001000, 0x1001], output: true },
  ],
  min_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: 0x1000 },
    { inputs: [0x01001000, 0x1000], output: 0x1000 },
    { inputs: [0x00001000, 0x1001], output: 0x1000 },
  ],
  max_euint32_euint16: [
    { inputs: [0x00001000, 0x1000], output: 0x1000 },
    { inputs: [0x01001000, 0x1000], output: 0x01001000 },
    { inputs: [0x00001000, 0x1001], output: 0x1001 },
  ],
  add_euint32_euint32: [{ inputs: [0x00321000, 0x00111000], output: 0x00432000 }],
  sub_euint32_euint32: [{ inputs: [0x00321000, 0x00111000], output: 0x00210000 }],
  mul_euint32_euint32: [{ inputs: [0x00321000, 0x00000020], output: 0x06420000 }],
  and_euint32_euint32: [
    { inputs: [0x00321000, 0x54000000], output: 0x00000000 },
    { inputs: [0x00321000, 0x54030000], output: 0x00020000 },
  ],
  or_euint32_euint32: [
    { inputs: [0x00321000, 0x54000000], output: 0x54321000 },
    { inputs: [0x00321000, 0x54030000], output: 0x54331000 },
  ],
  xor_euint32_euint32: [
    { inputs: [0x00321000, 0x54000000], output: 0x54321000 },
    { inputs: [0x00321000, 0x54030000], output: 0x54311000 },
  ],
  eq_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: true },
    { inputs: [0x00321000, 0x00321001], output: false },
  ],
  ne_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: false },
    { inputs: [0x00321000, 0x00321001], output: true },
  ],
  ge_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: true },
    { inputs: [0x00321000, 0x00321001], output: false },
    { inputs: [0x00321000, 0x00320fff], output: true },
  ],
  gt_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: false },
    { inputs: [0x00321000, 0x00321001], output: false },
    { inputs: [0x00321000, 0x00320fff], output: true },
  ],
  le_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: true },
    { inputs: [0x00321000, 0x00321001], output: true },
    { inputs: [0x00321000, 0x00320fff], output: false },
  ],
  lt_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: false },
    { inputs: [0x00321000, 0x00321001], output: true },
    { inputs: [0x00321000, 0x00320fff], output: false },
  ],
  min_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: 0x00321000 },
    { inputs: [0x00321000, 0x00321001], output: 0x00321000 },
    { inputs: [0x00321000, 0x00320fff], output: 0x00320fff },
  ],
  max_euint32_euint32: [
    { inputs: [0x00321000, 0x00321000], output: 0x00321000 },
    { inputs: [0x00321000, 0x00321001], output: 0x00321001 },
    { inputs: [0x00321000, 0x00320fff], output: 0x00321000 },
  ],
  add_euint32_uint32: [{ inputs: [0x00342000, 0x00321000], output: 0x00663000 }],
  add_uint32_euint32: [{ inputs: [0x00342000, 0x00321000], output: 0x00663000 }],
  sub_euint32_uint32: [{ inputs: [0x00342000, 0x00321000], output: 0x00021000 }],
  sub_uint32_euint32: [{ inputs: [0x00342000, 0x00321000], output: 0x00021000 }],
  mul_euint32_uint32: [{ inputs: [0x00342000, 0x00000100], output: 0x34200000 }],
  mul_uint32_euint32: [{ inputs: [0x00342000, 0x00000100], output: 0x34200000 }],
  div_euint32_uint32: [{ inputs: [0x00342000, 0x00000100], output: 0x00003420 }],
  rem_euint32_uint32: [{ inputs: [0x00342039, 0x00000100], output: 0x00000039 }],
  eq_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: true },
    { inputs: [0x00342000, 0x00342001], output: false },
  ],
  eq_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: true },
    { inputs: [0x00342000, 0x00342001], output: false },
  ],
  ne_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: false },
    { inputs: [0x00342000, 0x00342001], output: true },
  ],
  ne_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: false },
    { inputs: [0x00342000, 0x00342001], output: true },
  ],
  ge_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: true },
    { inputs: [0x00342000, 0x00342001], output: false },
    { inputs: [0x00342000, 0x00341fff], output: true },
  ],
  ge_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: true },
    { inputs: [0x00342000, 0x00342001], output: false },
    { inputs: [0x00342000, 0x00341fff], output: true },
  ],
  gt_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: false },
    { inputs: [0x00342000, 0x00342001], output: false },
    { inputs: [0x00342000, 0x00341fff], output: true },
  ],
  gt_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: false },
    { inputs: [0x00342000, 0x00342001], output: false },
    { inputs: [0x00342000, 0x00341fff], output: true },
  ],
  le_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: true },
    { inputs: [0x00342000, 0x00342001], output: true },
    { inputs: [0x00342000, 0x00341fff], output: false },
  ],
  le_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: true },
    { inputs: [0x00342000, 0x00342001], output: true },
    { inputs: [0x00342000, 0x00341fff], output: false },
  ],
  lt_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: false },
    { inputs: [0x00342000, 0x00342001], output: true },
    { inputs: [0x00342000, 0x00341fff], output: false },
  ],
  lt_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: false },
    { inputs: [0x00342000, 0x00342001], output: true },
    { inputs: [0x00342000, 0x00341fff], output: false },
  ],
  min_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: 0x00342000 },
    { inputs: [0x00342000, 0x00342001], output: 0x00342000 },
    { inputs: [0x00342000, 0x00341fff], output: 0x00341fff },
  ],
  min_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: 0x00342000 },
    { inputs: [0x00342000, 0x00342001], output: 0x00342000 },
    { inputs: [0x00342000, 0x00341fff], output: 0x00341fff },
  ],
  max_euint32_uint32: [
    { inputs: [0x00342000, 0x00342000], output: 0x00342000 },
    { inputs: [0x00342000, 0x00342001], output: 0x00342001 },
    { inputs: [0x00342000, 0x00341fff], output: 0x00342000 },
  ],
  max_uint32_euint32: [
    { inputs: [0x00342000, 0x00342000], output: 0x00342000 },
    { inputs: [0x00342000, 0x00342001], output: 0x00342001 },
    { inputs: [0x00342000, 0x00341fff], output: 0x00342000 },
  ],
};
