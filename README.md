# Posit-sh's backend

This repository stores Posit-sh's backend code, or more specifically, Posit-sh's API REST. It's written in Rust on top of `actix-web`, using `tokio-postgres` and `deadpool-postgres` to communicate with a **Postgres** database.

## Available ENDPOINTS

- GET /posits
- GET /postits/{id}
- POST /posits
- PATCH /posits/{id}
- DELETE /posits/{id}

## Instructions to run locally

1. Clone this repository.
```bash
git clone https://github.com/ticianomorvan/positsh-backend.git
```

2. Create an user for your Postgres database.
```sql
CREATE USER username WITH PASSWORD 'password';
```

3. Create your Postgres database.
```sql
CREATE DATABASE database_name OWNER username;
```

4. Initialize your database.
```bash
psql -f sql/schema.sql database_name
```

5. Grant privileges to your user. (you can give them all or limit them to the database)
```sql
GRANT ALL PRIVILEGES ON SCHEMA psh TO username;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA psh TO username;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA psh TO username;
```

6. Create an `.env` file.
```bash
SERVER_ADDRESS=127.0.0.1:8080
POSTGRES.USER=username
POSTGRES.PASSWORD=password
POSTGRES.HOST=127.0.0.1
POSTGRES.PORT=5432
POSTGRES.DBNAME=database_name
POSTGRES.POOL.MAX_SIZE=16
```

7. Run the HTTP server.
```bash
cargo run
```