/*
  Warnings:

  - You are about to drop the column `status` on the `Dapp` table. All the data in the column will be lost.
  - Made the column `address` on table `Dapp` required. This step will fail if there are existing NULL values in that column.
  - Made the column `chainId` on table `Dapp` required. This step will fail if there are existing NULL values in that column.

*/
UPDATE "Dapp" d SET "chainId" = 1 WHERE d."chainId" is NULL;
UPDATE "Dapp" d SET "address" = '' WHERE d."address" is NULL;

-- AlterTable
ALTER TABLE "Dapp" DROP COLUMN "status",
ALTER COLUMN "address" SET NOT NULL,
ALTER COLUMN "chainId" SET NOT NULL;

-- DropEnum
DROP TYPE "DappStatus";
