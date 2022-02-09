const { debug, log } = require("./debug");
const getAppWebsocket = require("./appWebsocket");
const { ZOME } = require("./constants");

async function hcCall(fnName, params) {
  const startTime = Date.now();

  const { socket, cellId } = await getAppWebsocket();
  console.log("Calling the holochain conductor: ", cellId, ZOME, fnName, params);
  console.log(cellId)
  try {
    const result = await socket.callZome({
      cap: null,
      cell_id: cellId,
      zome_name: ZOME,
      fn_name: fnName,
      payload: params,
      provenance: cellId[1],
    });

    console.log(
      `[REQUEST] Response from the holochain conductor in : ${
        (Date.now() - startTime) / 1000
      }`,
      result
    );

    return result;
  } catch (e) {
    if (
      e.data &&
      e.data.type == "internal_error" &&
      e.data.data.includes("the source chain head has moved")
    ) {
      log(
        "Zome function resulted in race condition where the chain head moved, retrying"
      );
      return hcCall(fnName, params);
    } else throw e;
  }
}

module.exports = hcCall;
