/*
  Warnings:

  - You are about to drop the `Chain` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE "Dapp" DROP CONSTRAINT "Dapp_chainId_fkey";

-- DropTable
DROP TABLE "Chain";
