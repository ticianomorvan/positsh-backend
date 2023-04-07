UPDATE psh.posits SET (title, content) = ($2, $3) WHERE id = $1
RETURNING $table_fields;