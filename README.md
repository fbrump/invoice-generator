# Invoice Generator

In this repository I want to share my knowledge and study around Rust with a real project to boost my boundaries far away

# Stack

- [Rust](https://rust-lang.org/)
- [Axum](https://docs.rs/axum/latest/axum/)
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [PostgreSQL](https://www.postgresql.org/)
- [Podman](https://podman.io/)

# How To

For local development, we can use this command below:

```bash
cargo watch -x run
``` 

## Endpoint Tests

Using the [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client&ssr=false#overview) on the VS Code we can execute some endpoint tests.

- transactions.http

# Databse

Using the compose fiel, we will raise a database local as container.

```bash

podman-compose up
# OR
podman-compose up postgres pgadmin
```

There is a PostgreSQL container a.k.a postgres.

## SQLx

Manage migrations using the SQLx.

```bash

sqlx database create

```

Then, to update the migration files, add a new file with the command below.

```bash

sqlx migrate run

```
In oposite of this, we could execute the revert.

```bash
sqlx migrate revert
```

---
As an optional command is to execute:

```bash
sqlx migrate add -r [context]

```
As a result, it will create two file e.g. `.._[context].up.sql` and `.._[context].up.sql`. 

---

## Management

Also, a container with the pgAdmin that you can check the database and manager it.

- http://0.0.0.0:5050/browser/

Just click on this link before, then go to Servers and setup a new server:

- name: db-container
- hostname: app-db _(container name)_
- port: 5432 _(default)_
- username: postgres
- password: postgres

> You can check the `compose.yaml` for these data update.

After to connect on the server, go to Databases -> app_db -> Schemas -> public -> Tables.

- _sqlx_migrations -> Responsible to keep the migration history
- transactions

**TBD**