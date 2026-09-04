import { readFileSync } from 'node:fs';
import { z } from 'zod';

const OUTPUT_PATH = /^\.\/(?:(?!\.{1,2}(?:\/|$))[A-Za-z0-9._-]+\/)*(?!\.{1,2}$)[A-Za-z0-9._-]+$/;
const IDENTIFIER = /^[A-Za-z_$][A-Za-z0-9_$]*$/;

export const valueKinds = ['function', 'string', 'number', 'bigint', 'boolean', 'object', 'array'] as const;

const commentSchema = z.union([z.string(), z.array(z.string())]);
const linesSchema = z.union([z.string().min(1), z.array(z.string()).min(1)]);
const valueKindSchema = z.enum(valueKinds);
const comments = { '//': commentSchema.optional() };

const typeExportEntrySchema = z
  .object({ ...comments, name: z.string().regex(IDENTIFIER), comment: z.array(z.string()).optional() })
  .strict();

const valueExportEntrySchema = z
  .object({
    ...comments,
    name: z.string().regex(IDENTIFIER),
    kind: valueKindSchema,
    member: z.record(z.string().regex(IDENTIFIER), valueKindSchema).optional(),
    element: valueKindSchema.optional(),
    comment: z.array(z.string()).optional(),
  })
  .strict()
  .superRefine((entry, context) => {
    if (entry.kind !== 'array' && entry.element !== undefined) {
      context.addIssue({ code: 'custom', path: ['element'], message: "is valid only when kind is 'array'" });
    }
    if (entry.kind === 'array' && entry.member !== undefined) {
      context.addIssue({ code: 'custom', path: ['member'], message: "cannot be combined with kind 'array'" });
    }
  });

const exportBlockSchema = z.discriminatedUnion('typeOnly', [
  z
    .object({
      ...comments,
      comment: z.array(z.string()).optional(),
      module: z.string().min(1),
      typeOnly: z.literal(true),
      exports: z.array(typeExportEntrySchema).min(1),
    })
    .strict(),
  z
    .object({
      ...comments,
      comment: z.array(z.string()).optional(),
      module: z.string().min(1),
      typeOnly: z.literal(false).optional().default(false),
      exports: z.array(valueExportEntrySchema).min(1),
    })
    .strict(),
]);

const exportManifestSchema = z
  .object({
    $schema: z.string().min(1).optional(),
    ...comments,
    packageSpecifier: z.string().min(1),
    printWidth: z.number().int().min(40).max(400).optional().default(120),
    outputs: z
      .object({
        exports: z.string().regex(OUTPUT_PATH, 'must be a safe path relative to the export manifest'),
        testConsumers: z
          .object({
            cjs: z.string().regex(OUTPUT_PATH, 'must be a safe path relative to the export manifest').optional(),
            esm: z.string().regex(OUTPUT_PATH, 'must be a safe path relative to the export manifest').optional(),
          })
          .strict()
          .refine((tests) => tests.cjs !== undefined || tests.esm !== undefined, {
            message: 'must define at least one of cjs or esm',
          }),
        // The package's own harness test project, which runs vitest rather than node:test — so this
        // output is emitted in vitest's dialect. Optional: a package need not have one.
        test: z.string().regex(OUTPUT_PATH, 'must be a safe path relative to the export manifest').optional(),
      })
      .strict(),
    blocks: z.array(exportBlockSchema).min(1),
    dummies: z
      .object({
        ...comments,
        preamble: z
          .array(
            z
              .object({
                ...comments,
                name: z.string().regex(IDENTIFIER),
                type: z.string().min(1),
                expression: z.string().min(1),
              })
              .strict(),
          )
          .superRefine(uniqueBy('name')),
        values: z
          .array(z.object({ ...comments, type: z.string().regex(IDENTIFIER), dummy: linesSchema }).strict())
          .superRefine(uniqueBy('type')),
      })
      .strict(),
  })
  .strict()
  .superRefine((manifest, context) => {
    const exports = manifest.blocks.flatMap((block) => block.exports.map((entry) => entry.name));
    addDuplicates(exports, context, ['blocks'], 'export names must be unique across all blocks');

    const outputPaths = [
      manifest.outputs.exports,
      manifest.outputs.testConsumers.cjs,
      manifest.outputs.testConsumers.esm,
      manifest.outputs.test,
    ].filter((path): path is string => path !== undefined);
    addDuplicates(outputPaths, context, ['outputs'], 'output paths must be unique');

    const declaredTypes = manifest.blocks
      .filter((block) => block.typeOnly)
      .flatMap((block) => block.exports.map((entry) => entry.name));
    const sampledTypes = manifest.dummies.values.map((value) => value.type);
    const missing = declaredTypes.filter((name) => !sampledTypes.includes(name));
    const orphaned = sampledTypes.filter((name) => !declaredTypes.includes(name));
    if (missing.length > 0) {
      context.addIssue({
        code: 'custom',
        path: ['dummies', 'values'],
        message: `missing samples for exported types: ${missing.join(', ')}`,
      });
    }
    if (orphaned.length > 0) {
      context.addIssue({
        code: 'custom',
        path: ['dummies', 'values'],
        message: `samples do not match exported types: ${orphaned.join(', ')}`,
      });
    }
  });

export type ValueKind = (typeof valueKinds)[number];
export type ExportManifest = z.infer<typeof exportManifestSchema>;
export type ExportBlock = ExportManifest['blocks'][number];
export type ExportEntry = ExportBlock['exports'][number];

export class ExportManifestValidationError extends Error {
  constructor(file: string, message: string) {
    super(`${file}: ${message}`);
    this.name = 'ExportManifestValidationError';
  }
}

export function parseExportManifest(value: unknown, file = 'export.manifest.json'): ExportManifest {
  const result = exportManifestSchema.safeParse(value);
  if (!result.success) throw new ExportManifestValidationError(file, z.prettifyError(result.error));
  return result.data;
}

export function loadExportManifest(file: string): ExportManifest {
  let value: unknown;
  try {
    value = JSON.parse(readFileSync(file, 'utf8')) as unknown;
  } catch (error) {
    throw new ExportManifestValidationError(file, `unable to parse JSON: ${errorMessage(error)}`);
  }
  return parseExportManifest(value, file);
}

function uniqueBy<Key extends string>(key: Key) {
  return (values: readonly Record<Key, string>[], context: z.RefinementCtx): void => {
    addDuplicates(
      values.map((value) => value[key]),
      context,
      [],
      `must not contain duplicate '${key}' values`,
    );
  };
}

function addDuplicates(
  values: readonly string[],
  context: z.RefinementCtx,
  path: readonly PropertyKey[],
  message: string,
): void {
  if (new Set(values).size !== values.length) context.addIssue({ code: 'custom', path: [...path], message });
}

function errorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}
