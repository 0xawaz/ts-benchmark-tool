# tsbtool by 0xawaz for TimeScaleDB

## Install binaries

Make sure you have docker and docker-compose [installed](https://docs.docker.com/engine/install/).

## Run Using docker-compose
We prefer to use docker-compose to avoid any dependencies incompatibilities, and all the local mess.

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
