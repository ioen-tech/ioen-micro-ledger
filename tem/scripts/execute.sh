#!/bin/bash

APP_PORT=${HC_PORT:-9990}
RUST_LOG=${DEBUG:-info}

sleep 5 && CONDUCTOR_URL=ws://localhost:$APP_PORT APPLICATION_ID=$APPLICATION_ID node /tem/express-relayer/src/index.js&

### Check if a directory does not exist ###
if [ ! -d "/tem/databases/done" ] 
then
    # This is the first run: create sandbox and run
    hc sandbox create --root /tem/databases -d=sandbox network --bootstrap https://bootstrap-staging.holo.host/ quic 
    hc sandbox call install-app-bundle --app-id=tem /tem/tem.happ 
    hc sandbox call list-active-apps 
    mkdir /tem/databases/done
    RUST_LOG=$RUST_LOG hc sandbox run --ports=$APP_PORT
else
    # This is the second run: reset keystore and run

    lair-keystore -d /tem/databases/sandbox/keystore &
    sleep 1
    RUST_LOG=$RUST_LOG holochain -c /tem/databases/sandbox/conductor-config.yaml 
fi