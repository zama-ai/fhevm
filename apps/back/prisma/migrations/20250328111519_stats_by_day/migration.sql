-- CreateEnum
CREATE TYPE "StatsType" AS ENUM ('SYMBOLIC', 'FHE');

-- AlterTable
ALTER TABLE "DappStat" ADD COLUMN     "day" INTEGER NOT NULL DEFAULT 1,
ADD COLUMN     "month" INTEGER NOT NULL DEFAULT 1,
ADD COLUMN     "type" "StatsType" NOT NULL DEFAULT 'SYMBOLIC',
ADD COLUMN     "year" INTEGER NOT NULL DEFAULT 2025;

-- Update the day, month, and year based on timestamp
UPDATE "DappStat"
SET 
    "day" = EXTRACT(DOY FROM "timestamp"),
    "month" = EXTRACT(MONTH FROM "timestamp"),
    "year" = EXTRACT(YEAR FROM "timestamp");


