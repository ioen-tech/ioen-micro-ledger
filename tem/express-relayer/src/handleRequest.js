const { debug, log } = require("./debug");
const { createContract, getContracts } = require("./handlers/contract");
const { createReceipt, getReceipts } = require("./handlers/receipt");
const { createMessage, getMessages } = require("./handlers/message");

async function handleRequest(req, res) {
  const url = req.originalUrl.split("/")[1];

  const startTime = Date.now();
  log(`[REQUEST] Received request: `, url, req.body);

  let completed = false;

  executeRequest(req.method, url, req.headers, req.body)
    .then((result) => {
      log(
        `[REQUEST] Request result in ${
          (Date.now() - startTime) / 1000
        }s: Successfully handled request, result: `,
        result
      );

      if (completed) return;

      completed = true;

      res.status(200);
      res.json(result);
    })
    .catch((error) => {
      log(
        `[REQUEST] Request result in ${
          (Date.now() - startTime) / 1000
        }s: Error handling the request: `,
        error
      );

      if (completed) return;

      res.status(500);

      try {
        res.json(error.data);
      } catch (e) {
        res.json({
          Error: JSON.stringify(error),
        });
      }
    });
}

async function executeRequest(method, url, headers, body) {
  if (method === "POST") {
    return handleCreate(url, body);
  } else {
    return handleGet(url, headers);
  }
}

async function handleCreate(url, body) {
  debug("Received new post request: ", url, "with id ", body.id);

  // Use the appropriate create function for the object type
  let result;
  switch (url) {
    case "Contract":
      result = await createContract(body);
      break;
    case "GetContracts":
      result = await getContracts(body);
      break;
    case "Receipt":
      result = await createReceipt(body);
      break;
    case "GetReceipts":
      result = await getReceipts(body);
      break;
    case "Message":
      result = await createMessage(body);
      break;
    case "GetMessages":
      result = await getMessages(body);
      break;
    default:
      throw new Error(JSON.stringify({ Error: "Wrong object type to create" }));
  }

  debug("Relaying result of the request: ", result);
  return result;
}

async function handleGet(url, headers) {
  switch (url) {
    default:
      break;
  }
}

module.exports = handleRequest;
