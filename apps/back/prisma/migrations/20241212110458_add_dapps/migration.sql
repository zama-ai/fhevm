-- CreateEnum
CREATE TYPE "DappStatus" AS ENUM ('DRAFT', 'DEPLOYING', 'LIVE', 'ARCHIVED', 'DELETED');

-- CreateTable
CREATE TABLE "Dapp" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "address" TEXT,
    "status" "DappStatus" NOT NULL DEFAULT 'DRAFT',
    "teamId" TEXT NOT NULL,
    "updatedAt" TIMESTAMP(3) NOT NULL,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Dapp_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "Dapp" ADD CONSTRAINT "Dapp_teamId_fkey" FOREIGN KEY ("teamId") REFERENCES "Team"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
