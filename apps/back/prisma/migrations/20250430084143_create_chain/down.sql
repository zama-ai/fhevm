-- Remove ForeignKey
ALTER TABLE "Dapp" DROP CONSTRAINT IF EXISTS "Dapp_chainId_fkey";

-- Delete table
DELETE TABLE IF EXISTS "Chain";

