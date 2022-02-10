const ZOME = "ledger";
const DEBUG = process.env.DEBUG;
const APPLICATION_ID = process.env.APPLICATION_ID;
const RELAYER_PORT = process.env.PORT ? process.env.PORT : 8888;
const IS_CLOUD =
  process.env.IS_CLOUD === "true" || process.env.IS_CLOUD === "1";
const CONDUCTOR_URL = process.env.CONDUCTOR_URL;

console.log(APPLICATION_ID)
if (!APPLICATION_ID) throw new Error('APPLICATION_ID is not set: you must set it as an environment variable in the docker-compose')

module.exports = {
  APPLICATION_ID,
  RELAYER_PORT,
  CONDUCTOR_URL,
  ZOME,
  DEBUG,
  IS_CLOUD,
};
