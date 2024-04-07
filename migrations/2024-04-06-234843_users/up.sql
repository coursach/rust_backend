CREATE TABLE "users" (
	"Id"	INTEGER,
	"Name"	TEXT NOT NULL,
	"Surname"	TEXT NOT NULL,
	"Password"	TEXT NOT NULL,
	"Email"	TEXT NOT NULL,
	"Role"	INTEGER NOT NULL,
	PRIMARY KEY("Id" AUTOINCREMENT),
	FOREIGN KEY("Role") REFERENCES "role"("Id") ON DELETE CASCADE
)