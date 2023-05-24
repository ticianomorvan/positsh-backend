# positsh-backend
Posit-sh was supposed to be a place for people to upload their summaries, notes and knowledge in a similar way they do it in real life with paper "posits". I left this project in favor of other responsabilities, but I would like to make this repository public and open-source for anybody that's learning Rust and wants to see how a REST API with Actix can look like.

## Features
- User authentication (JWT)
- User resources' protected by ID validation
- CRUD endpoints for Posits (kind of posts)
- PostgreSQL database accesible with SQLx

## Running it
1. Before running anything, populate and `.env` file with the structure detailed in `.env.example`.
2. With `docker-engine` and `docker-compose` installed, run `docker-compose up -d` in the root directory.
3. Then, you can run the API with `cargo run` or `cargo-watch`, if you have installed it.
