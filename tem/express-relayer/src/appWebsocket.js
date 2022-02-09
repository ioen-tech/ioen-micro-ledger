const { AppWebsocket } = require("@holochain/client");

const { CONDUCTOR_URL } = require("./constants");
let socket = undefined;
let cellId = undefined;

// Connect to the holochain conductor and store the connection
// Retrying is handled upstream
async function getAppWebsocket() {
  if (socket) return { socket, cellId };

  try {
    socket = await AppWebsocket.connect(CONDUCTOR_URL);

    console.log("Successfully connected to the conductor!");

    cellId = await getCellId();

    return { socket, cellId };
  } catch (e) {
    console.log(e);
    throw new Error(
      "Received error when trying to connect to the conductor, ",
      e
    );
  }
}

function getEnvCellId() {
  const cellId = JSON.parse(process.env.CELL_ID);

  return cellId.map((id) => Buffer.from(id));
}

async function getCellId() {
  if (process.env.CELL_ID) return getEnvCellId();

  const { socket } = await getAppWebsocket();

  const appInfo = await socket.appInfo({ installed_app_id: "tem" });

  return appInfo.cell_data[0].cell_id;
}

module.exports = getAppWebsocket;
