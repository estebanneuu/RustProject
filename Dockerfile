FROM rust

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/actix-tera-test

COPY . .

RUN cargo install --path .

CMD bash -c "diesel setup && actix-tera-test"