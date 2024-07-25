# `tsbtool` by 0xawaz for Timescale

## Install binaries

Make sure you have docker and docker-compose [installed](https://docs.docker.com/engine/install/).

If you need an ansible role for docker, please check [here](https://github.com/0xawaz/awaz-penumbra-testnet/blob/main/infra/roles/docker/tasks/main.yml).

## Quickstart

Please follow these steps to get tbstool up and running and get your benchmark stats:

```sh
# get project
git clone git@github.com:0xawaz/ts-benchmark-tool.git
cd ts-benchmark-tool/tsbtool

# get data
wget -O migrations/TimescaleDB_coding_assignment-RD_eng_setup.tar.gz "https://www.dropbox.com/s/17mr38w21yhgjjl/TimescaleDB_coding_assignment-RD_eng_setup.tar.gz?dl=1" \
&& tar -xzvf migrations/TimescaleDB_coding_assignment-RD_eng_setup.tar.gz -C migrations/ \
&& rm migrations/TimescaleDB_coding_assignment-RD_eng_setup.tar.gz

# replace password, you can also change other env vars if you wish
PASSWORD="your_password_here"
sed "s/<REPLACE-ME-WITH-YOUR-PASSWOD>/${PASSWORD}/" .env-example > .env

# populate env vars
source .env

# run program
docker-compose up -d

# check output
docker logs -f tsbtool

# cleanup
docker-compose down
```

Expected output looks like:

```sh
---> Reading CSV file ...
---> Start distributing queries among workers ...
---> Calculate benchmark statistics ...

---------------------------- Benchmark Stats ----------------------------
Number of queries run: 200
Total processing time: 3.681774081s
Minimum query time: 10.089875ms
Median query time: 16.913437ms
Average query time: 18.40887ms
```

Note that we wait for the database and data migration to be ready, this should take less than 15s depending on other factors like networking ...

Please contact [0xawaz](https://t.me/oxawaz) if you get any unexpected behavior.

## Dev Deep Dive

We are using docker-compose to avoid any dependencies incompatibilities, and because we don't like a local mess ;)

### Set up database and Run migration scripts

```sh
# populate env vars
source .env

# run containers - runs timescaledb then migrations scripts
docker-compose up -d db
docker-compose up -d migrations

# verify
docker ps
docker exec -it db bash
  $ psql -U postgres -d homework
  $ \dt

# cleanup 
docker-compose down
```

### Set up `tsbtool` Query Tool

#### Workflow

```sh
inputs  -> read csv |->  distribute work  -> outputs
                    |-> process query
                    |-> extract hostname
```

#### Compile and run

```sh
# compile locally and run binary
cargo update
cargo build
./target/debug/tsbtool migrations/query_params.csv --workers 4

# build docker image
docker build -t 0xawaz/tsbtool:0.1.0 .

# run and verify container
docker run --rm -it 0xawaz/tsbtool:0.1.0 --version
docker run --rm -it 0xawaz/tsbtool:0.1.0 --help

# run tsbtool and get bench-stats
docker run --rm -it 0xawaz/tsbtool:0.1.0 /app/tsbtool /app/query_params.csv --workers 4

# run using docker-compose
docker-compose up -d

# cleanup
docker-compose down
```

## To improve

* Development
    * Handle CSV as either STDIN or via a flag with the filename.
    * Provide additional benchmark statistics.
    * Use connection pool like deadpool-postgres to manage database connections efficiently.
    * Add Unit/functional tests. More case to test Error handling.

* Automation (CI/CD)
    * Use Github Actions to automate image build and publish.
    * Set stateless config for multiple environments (dev/staging/prod).

* Performence
    * loadtest `tsbtool` to check limitations.

* Security
    * Scan all used and created docker images and binaries and patch vulnerabilities if exists.
