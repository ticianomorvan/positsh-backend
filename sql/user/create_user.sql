INSERT INTO psh.users (fullname, username, password, email)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;