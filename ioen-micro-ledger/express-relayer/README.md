# express-relayer

The purpose of this component is to wrap holochain conductor calls as a REST API

## Usage

Run this:

``` bash
npm install
npm start
```

Or if you want to change the configuration parameters:

``` bash
IS_MARKET=true CONDUCTOR_URL=ws://localhost:8888 RELAYER_PORT=3000 node index.js
```
