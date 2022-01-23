const hcCall = require("../hcCall");
const { parseTimestampedEntries } = require("./utils");
const { encode, decode } = require("@msgpack/msgpack");

async function getContracts(filter) {
  const contracts = await hcCall("get_contracts", filter);
  let parsedContracts = parseTimestampedEntries(contracts);

  parsedContracts = parsedContracts.map((contract) => ({
    ...contract,
    payload: decode(contract.payload),
  }));

  return {
    Contracts: parsedContracts,
  };
}

async function createContract(contract) {
  const address = await hcCall("create_contract", {
    ...contract,
    payload: encode(contract.payload),
  });
  return { ContractAddress: address };
}

module.exports = {
  createContract,
  getContracts,
};
