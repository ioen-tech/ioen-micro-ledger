const hcCall = require("../hcCall");
const { parseTimestampedEntries } = require("./utils");
const { encode, decode } = require("@msgpack/msgpack");

async function getReceipts(filter) {
  const receipts = await hcCall("get_receipts", filter);
  let parsedReceipts = parseTimestampedEntries(receipts);

  parsedReceipts = parsedReceipts.map((receipt) => ({
    ...receipt,
    payload: decode(receipt.payload),
  }));

  return {
    Receipts: parsedReceipts,
  };
}

async function createReceipt(receipt) {
  const address = await hcCall("create_receipt", {
    ...receipt,
    payload: encode(receipt.payload),
  });
  return { ReceiptAddress: address };
}

module.exports = {
  createReceipt,
  getReceipts,
};
