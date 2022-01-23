const hcCall = require("../hcCall");
const { parseTimestampedEntries } = require("./utils");
const { encode, decode } = require("@msgpack/msgpack");

async function getMessages(filter) {
  const messages = await hcCall("get_messages", filter);
  let parsedmessages = parseTimestampedEntries(messages);

  parsedmessages = parsedmessages.map((message) => ({
    ...message,
    payload: decode(message.payload),
  }));

  return {
    Messages: parsedmessages,
  };
}

async function createMessage(message) {
  const address = await hcCall("create_message", {
    ...message,
    payload: encode(message.payload),
  });
  return { MessageAddress: address };
}

module.exports = {
  createMessage,
  getMessages,
};
