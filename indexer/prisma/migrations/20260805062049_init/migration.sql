-- CreateEnum
CREATE TYPE "OrderStatus" AS ENUM ('Open', 'Partial', 'filled', 'Cancelled', 'Settled');

-- CreateTable
CREATE TABLE "Trade" (
    "id" TEXT NOT NULL,
    "signature" TEXT NOT NULL,
    "marketAddress" TEXT NOT NULL,
    "price" DECIMAL(28,6) NOT NULL,
    "quantity" DECIMAL(28,9) NOT NULL,
    "side" TEXT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "Trade_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Candle" (
    "id" TEXT NOT NULL,
    "marketAddress" TEXT NOT NULL,
    "resolution" TEXT NOT NULL,
    "timestamp" TIMESTAMP(3) NOT NULL,
    "open" DECIMAL(28,9) NOT NULL,
    "high" DECIMAL(28,9) NOT NULL,
    "low" DECIMAL(28,9) NOT NULL,
    "close" DECIMAL(28,9) NOT NULL,
    "volume" DECIMAL(28,9) NOT NULL,

    CONSTRAINT "Candle_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "Order" (
    "id" TEXT NOT NULL,
    "orderId" TEXT NOT NULL,
    "clientOrderId" TEXT NOT NULL,
    "marketAddress" TEXT NOT NULL,
    "ownerAddress" TEXT NOT NULL,
    "side" TEXT NOT NULL,
    "price" INTEGER NOT NULL,
    "baseLots" INTEGER NOT NULL,
    "filledLots" INTEGER NOT NULL DEFAULT 0,
    "status" "OrderStatus" NOT NULL DEFAULT 'Open',
    "placedAt" TIMESTAMP(3) NOT NULL,
    "cancelledAt" TIMESTAMP(3),
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "settledAt" TIMESTAMP(3),

    CONSTRAINT "Order_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Trade_signature_key" ON "Trade"("signature");

-- CreateIndex
CREATE INDEX "Trade_marketAddress_timestamp_idx" ON "Trade"("marketAddress", "timestamp");

-- CreateIndex
CREATE UNIQUE INDEX "Candle_marketAddress_resolution_timestamp_key" ON "Candle"("marketAddress", "resolution", "timestamp");

-- CreateIndex
CREATE UNIQUE INDEX "Order_orderId_key" ON "Order"("orderId");

-- CreateIndex
CREATE INDEX "Order_ownerAddress_marketAddress_idx" ON "Order"("ownerAddress", "marketAddress");

-- CreateIndex
CREATE INDEX "Order_marketAddress_status_idx" ON "Order"("marketAddress", "status");

-- CreateIndex
CREATE INDEX "Order_orderId_idx" ON "Order"("orderId");
