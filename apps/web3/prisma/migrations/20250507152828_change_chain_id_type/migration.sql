BEGIN;
ALTER TABLE "FheEvent" ADD COLUMN IF NOT EXISTS "chainId_int" INT NULL;

UPDATE "FheEvent"
SET "chainId_int" = CAST("chainId" AS INTEGER)
WHERE "chainId_int" IS NULL;

ALTER TABLE "FheEvent" ALTER COLUMN "chainId_int" SET NOT NULL;
ALTER TABLE "FheEvent" DROP COLUMN IF EXISTS "chainId";
ALTER TABLE "FheEvent" RENAME COLUMN "chainId_int" TO "chainId";

END;
