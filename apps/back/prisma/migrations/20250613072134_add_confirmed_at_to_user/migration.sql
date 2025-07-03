-- AlterTable
ALTER TABLE "User" ADD COLUMN     "confirmedAt" TIMESTAMP(3);

-- Confirm all users created with invitation
UPDATE "User"
SET "confirmedAt" = now()
WHERE "confirmedAt" is null;