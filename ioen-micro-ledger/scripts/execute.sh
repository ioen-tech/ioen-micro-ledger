#!/bin/bash

APP_PORT=${HC_PORT:-9990}
RUST_LOG=${DEBUG:-info}

sleep 5 && CONDUCTOR_URL=ws://localhost:$APP_PORT APPLICATION_ID=$APPLICATION_ID node /ledger/express-relayer/src/index.js&

### Check if a directory does not exist ###
if [ ! -d "/ledger/databases/done" ] 
then
    # This is the first run: create sandbox and run
    hc sandbox create --root /ledger/databases -d=sandbox network --bootstrap https://bootstrap-staging.holo.host/ quic 
    hc sandbox call install-app-bundle --app-id=ledger /ledger/ledger.happ 
    hc sandbox call list-active-apps 
    mkdir /ledger/databases/done
    RUST_LOG=$RUST_LOG hc sandbox run --ports=$APP_PORT
else
    # This is the second run: reset keystore and run

    lair-keystore -d /ledger/databases/sandbox/keystore &
    sleep 1
    RUST_LOG=$RUST_LOG holochain -c /ledger/databases/sandbox/conductor-config.yaml 
fi