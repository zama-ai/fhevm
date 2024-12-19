-- DropForeignKey
ALTER TABLE "Dapp" DROP CONSTRAINT "Dapp_teamId_fkey";

-- AddForeignKey
ALTER TABLE "Dapp" ADD CONSTRAINT "Dapp_teamId_fkey" FOREIGN KEY ("teamId") REFERENCES "Team"("id") ON DELETE CASCADE ON UPDATE CASCADE;
