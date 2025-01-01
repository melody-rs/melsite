-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_User" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "github_id" INTEGER NOT NULL,
    "username" TEXT NOT NULL,
    "is_admin" BOOLEAN NOT NULL DEFAULT false
);
INSERT INTO "new_User" ("github_id", "id", "username") SELECT "github_id", "id", "username" FROM "User";
DROP TABLE "User";
ALTER TABLE "new_User" RENAME TO "User";
CREATE UNIQUE INDEX "User_github_id_key" ON "User"("github_id");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
