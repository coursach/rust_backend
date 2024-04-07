CREATE TABLE "codepromo" (
	"Id"	INTEGER,
	"Description"	TEXT NOT NULL,
	"IdSubscibe"	INTEGER NOT NULL,
	FOREIGN KEY("IdSubscibe") REFERENCES "subscribe"("Id") ON DELETE CASCADE,
	PRIMARY KEY("Id" AUTOINCREMENT)
)