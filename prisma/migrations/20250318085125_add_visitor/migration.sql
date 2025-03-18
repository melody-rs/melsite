-- CreateTable
CREATE TABLE "Visitor" (
    "hash" TEXT NOT NULL
);

-- CreateIndex
CREATE UNIQUE INDEX "Visitor_hash_key" ON "Visitor"("hash");
