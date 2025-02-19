-- CreateTable
CREATE TABLE "Snapshot" (
    "id" TEXT NOT NULL,
    "secondaryKey" TEXT NOT NULL UNIQUE,
    "content" TEXT NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "Snapshot_pkey" PRIMARY KEY ("id")
);
