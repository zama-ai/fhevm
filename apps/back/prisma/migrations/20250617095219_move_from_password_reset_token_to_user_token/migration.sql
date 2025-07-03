/*
  Warnings:

  - You are about to drop the `password_reset_tokens` table. If the table is not empty, all the data it contains will be lost.

*/
-- CreateEnum
CREATE TYPE "UserTokenType" AS ENUM ('RESET_PASSWORD', 'CONFIRM_EMAIL');

-- DropTable
DROP TABLE "password_reset_tokens";

-- CreateTable
CREATE TABLE "user_tokens" (
    "token_hash" TEXT NOT NULL,
    "user_id" TEXT NOT NULL,
    "expires_at" TIMESTAMP(3) NOT NULL,
    "token_type" "UserTokenType" NOT NULL,

    CONSTRAINT "user_tokens_pkey" PRIMARY KEY ("token_hash")
);

-- CreateIndex
CREATE UNIQUE INDEX "user_tokens_user_id_token_type_key" ON "user_tokens"("user_id", "token_type");
