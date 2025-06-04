-- CreateTable
CREATE TABLE IF NOT EXISTS "Chain" (
    "id" INTEGER NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT,
    "enabled" BOOLEAN NOT NULL DEFAULT false,

    CONSTRAINT "Chain_pkey" PRIMARY KEY ("id")
);

-- Note: there is no `ADD CONSTRAINT IF NOT EXISTS` because there is no way to assume its
-- properties. So, you should drop it if exists, and add again
ALTER TABLE "Dapp" DROP CONSTRAINT IF EXISTS "Dapp_chainId_fkey";
-- AddForeignKey
ALTER TABLE "Dapp" ADD CONSTRAINT "Dapp_chainId_fkey" FOREIGN KEY ("chainId") REFERENCES "Chain"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- Seed DB
INSERT INTO "Chain" ("id", "name", "description")
VALUES 
    ('1', 'Mainnet', 'Ethereum Mainnet'),
    ('11155111', 'Sepolia', 'Ethereum Sepolia'),
    ('123456', 'Local Testnet', NULL)
ON CONFLICT("id") DO NOTHING;