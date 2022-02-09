const { DEBUG, APPLICATION_ID } = require("./constants");

function debug(...debugArgs) {
  if (DEBUG)
    console.log(
      `[${APPLICATION_ID} at ${new Date().toLocaleString()}] Debug: `,
      debugArgs
    );
}

function log(...debugArgs) {
  console.log(
    `[${APPLICATION_ID} at ${new Date().toLocaleString()}] Log: `,
    debugArgs
  );
}

module.exports = { debug, log };
