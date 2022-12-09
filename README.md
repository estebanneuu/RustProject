# RustProject


## Members:

- Benoît GAILLARD
- Esteban NEUVY
- Wassim SALABLAB


## Structure

- Branch SERVER : Contains all sources relative to the server execution.

### How to run the server

```
$ sudo apt install postgres libpq-dev

$ cargo install diesel_cli --no-default-features --features postgres
```

Either run the docker-compose.yml to build a PGSQL server or run your own (if so, don't forget to edit the .env file with corresponding var)

```
$ diesel setup

$ cargo run
```

The server is now up and running on TCP port 8082

### How to run the client

```
$ ./beacon
```
