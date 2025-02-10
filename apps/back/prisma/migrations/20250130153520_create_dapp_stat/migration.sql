-- CreateTable
CREATE TABLE "DappStat" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL,
    "dappId" TEXT NOT NULL,
    "externalRef" TEXT NOT NULL,

    CONSTRAINT "DappStat_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "DappStat_externalRef_key" ON "DappStat"("externalRef");

-- AddForeignKey
ALTER TABLE "DappStat" ADD CONSTRAINT "DappStat_dappId_fkey" FOREIGN KEY ("dappId") REFERENCES "Dapp"("id") ON DELETE CASCADE ON UPDATE CASCADE;
