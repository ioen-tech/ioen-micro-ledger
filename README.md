# ***** ioen-ledger *****


## Pre-Requisites

Follow the instructions at https://developer.holochain.org/install/:
1. Install nixos, the Holochain operating system
2. Install the Holochain development tools, including the Rust compiler, crates, and environment

The version of Holochain we have currently built and tested to is version 0.0.122, 
```[nix-shell:~]$ holochain --version```

## Compiling

```bash
cd tem/dna
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
hc dna pack workdir/dna
hc app pack workdir/happ
```

This will output a `tem.happ` dna binary in the `tem/dna/workdir/happ` folder.

## Running the tests

After compiling, to run the tests:

```bash
cd tem/tests
npm install # if this is first run
npm test
```

These tests will automatically boot up the relayers and test their integration as well.

## Run agent directly with relayer

Open three terminals:

- First terminal:
```bash
hc sandbox generate 
RUST_LOG=debug holochain --interactive
```

If you're restarting the conductor, you may want to delete the database in disk, with `rm -rf <DB_PATH>`. You can see what database path you have configured by looking at your conductor-config.toml, default location in linux is `$HOME/.config/holochain/conductor-config.toml`

- Second terminal:
```bash
cd tem/scripts/install-instances
npm install # If this is the first run
PORTS=1111 DNA_PATH=~/projects/ioen-ledger/tem/dna/tem.dna.gz node index.js
```

The `DNA_PATH` should be adjusted to your needs, to point to the `men.dna.gz` file that you have locally

The `PORTS` variable is the port in which the instance of the app is going to be running.

- Third terminal:
```bash
cd tem/express-relayer
npm install # If this is the first run
CONDUCTOR_URL=ws://localhost:1111 DEBUG=true node index.js
```

## Creating the docker image

### Holochain Ledger

replace `.x` with the next incremental number, when you run the `docker build ...` command.

```bash
cd tem
docker build . -t ioen/ioen-ledger:ledger-0.x
```

### Push to docker hub

```bash
docker push ioen/ioen-ledger:ledger-0.x
```

### Docker compose files

Update the docker compose files, with the latest image tags
- docker/cloud/docker-compose.yml
- docker/edge-0/docker-compose.yml
- docker/edge-1/docker-compose.yml

## Test Network

To operate the small test network...

```bash
cd docker/cloud/
docker-compose up
```

Then in another terminal: 

```bash
cd docker/edge-0/
docker-compose up
```

Then in another terminal: 

```bash
cd docker/edge-1/
docker-compose up
```
