-- CreateTable
CREATE TABLE "FheEvent" (
    "chainId" TEXT NOT NULL,
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "callerAddress" TEXT NOT NULL,
    "blockNumber" INTEGER NOT NULL,
    "args" TEXT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "FheEvent_pkey" PRIMARY KEY ("id")
);
