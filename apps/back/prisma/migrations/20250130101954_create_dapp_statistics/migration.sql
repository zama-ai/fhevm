-- CreateTable
CREATE TABLE "DappStats" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL,
    "dappId" TEXT NOT NULL,

    CONSTRAINT "DappStats_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "DappStats" ADD CONSTRAINT "DappStats_dappId_fkey" FOREIGN KEY ("dappId") REFERENCES "Dapp"("id") ON DELETE CASCADE ON UPDATE CASCADE;
