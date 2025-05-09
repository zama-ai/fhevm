BEGIN;
ALTER TABLE "FheEvent" ADD COLUMN IF NOT EXISTS "chainId_txt" TEXT NULL;

UPDATE "FheEvent"
SET "chainId_txt" = CAST("chainId" AS TEXT)
WHERE "chainId_txt" IS NULL;

ALTER TABLE "FheEvent" ALTER COLUMN "chainId_txt" SET NOT NULL;
ALTER TABLE "FheEvent" DROP COLUMN IF EXISTS "chainId";
ALTER TABLE "FheEvent" RENAME COLUMN "chainId_txt" TO "chainId";

END;
