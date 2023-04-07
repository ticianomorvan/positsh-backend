INSERT INTO psh.posits (title, content, topic, author_name)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;