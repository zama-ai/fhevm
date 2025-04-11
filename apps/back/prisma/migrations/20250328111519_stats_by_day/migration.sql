-- CreateEnum
CREATE TYPE "StatsType" AS ENUM ('COMPUTATION', 'ENCRYPTION');

-- AlterTable
ALTER TABLE "DappStat"
ADD COLUMN     "day" INTEGER,
ADD COLUMN     "month" INTEGER,
ADD COLUMN     "year" INTEGER,
ADD COLUMN     "type" "StatsType" NOT NULL DEFAULT 'COMPUTATION';

-- Update the day, month, and year based on timestamp
UPDATE "DappStat"
SET 
    "day" = EXTRACT(DOY FROM "timestamp"),
    "month" = EXTRACT(MONTH FROM "timestamp"),
    "year" = EXTRACT(YEAR FROM "timestamp");


-- AlterTable to make columns NOT NULL
ALTER TABLE "DappStat"
ALTER COLUMN "day" SET NOT NULL,
ALTER COLUMN "month" SET NOT NULL,
ALTER COLUMN "year" SET NOT NULL;