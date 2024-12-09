import { ValueObject } from 'utils';
import { z } from 'zod';

const schema = z.string().regex(/^0x[a-fA-F0-9]{40}$/, 'Invalid Address');

export class Address extends ValueObject('Address', schema) {}
