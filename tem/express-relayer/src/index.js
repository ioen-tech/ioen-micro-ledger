const express = require("express");
const cors = require("cors");
const ws = require("ws");
const bodyParser = require("body-parser");

const handleRequest = require("./handleRequest");
const { RELAYER_PORT, CONDUCTOR_URL } = require("./constants");
const getAppWebsocket = require("./appWebsocket");
const { log } = require("./debug");
const {
  registerInTheNetwork,
} = require("./handlers/registerInTheNetwork");

const app = express();
// allow cross-origin requests
app.use(cors());
// parse requests as JSON automatically
app.use(bodyParser.json({ limit: "100Gb" }));

/**
 * Try to connect with the holochain conductor, and setup the relay connection
 */
async function setupRelayer() {
  log(`Trying to connect with the holochain conductor at ${CONDUCTOR_URL}...`);

  // Establish and save connection to the holochain conductor
  await getAppWebsocket();

  await registerInTheNetwork();

  log(`Connected! Listening on port ${RELAYER_PORT}`);

  // when we call `app.use` we aren"t setting the path for a specific api endpoint,
  // rather defining a namespace, meaning anything matching the path execute the handlers
  app.use("/", async (req, res) => {
    handleRequest(req, res);
  });

  const server = app.listen(RELAYER_PORT);
}

// Executes the given function and retries in an exponential backoff manner
async function executeOrRetry(action, retrySecs = 2) {
  try {
    await action();
  } catch (e) {
    console.error(e);
    console.log(`Retrying in ${retrySecs}s...`);

    setTimeout(() => executeOrRetry(action, retrySecs * 1.1), retrySecs * 1000);
  }
}

// Setup the relayer or retry with exponential backoff
executeOrRetry(() => setupRelayer());
