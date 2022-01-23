const { IS_CLOUD, APPLICATION_ID } = require("../constants");
const hcCall = require("../hcCall");
const { log } = require("../debug");

// If this is a market node, register the agent as such in the DHT
// If this is a bidder node, associate with the market present in the DHT
async function registerInTheNetwork() {
  if (IS_CLOUD) {
    log(`Registering as a cloud node...`);

    await hcCall("register_as_cloud", APPLICATION_ID);
    log("Successfully registered as a cloud node");
  } else {
    log("Associating with the cloud node...");

    await hcCall("associate_with_cloud", APPLICATION_ID);
    log("Successfully associated with the cloud node");
  }
}

module.exports = {
  registerInTheNetwork,
};
