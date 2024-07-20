# `tsbtool` by 0xawaz for Timescale

## Install binaries

Make sure you have docker and docker-compose [installed](https://docs.docker.com/engine/install/).

## Run Using docker-compose

We prefer to use docker-compose to avoid any dependencies incompatibilities, and all the local mess.

### Create .env file

We use .env to manage our environment variables and secrets, please replace with your values.

```bash
# Database ENV Variables
POSTGRES_USER=xxxxxx
POSTGRES_PASSWORD=xxxxxx
POSTGRES_DB_URL=xxxxxx
POSTGRES_DB=xxxxxx
```

### Setup database and Run migration scripts

```bash
# populate env vars
source .env

# run containers - runs timescaledb then migrations scripts
docker-compose up -d

# verify
# TODO

# cleanup 
docker-compose down
```

## Set up `tsbtool` Query tool

```bash
# compile locally and run binary
cargo update
cargo build
./target/debug/tsbtool migrations/query_params.csv --workers 8

# build docker image et run container
docker build -t 0xawaz/tsbtool:0.1.0 .
docker run --rm -it 0xawaz/tsbtool:0.1.0 --help
docker run --rm -it 0xawaz/tsbtool:0.1.0 --help

# docker-compose
docker-compose up -d
docker-compose down
```