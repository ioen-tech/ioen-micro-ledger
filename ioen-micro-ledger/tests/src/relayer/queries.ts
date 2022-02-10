import contract from "../objects/contract.json";
import receipt from "../objects/receipt.json";
import message from "../objects/message.json";
import fetch from "node-fetch";

export async function createContract(
  port,
  revisionId,
  sourceAddress,
  destinationAddress
) {
  return fetch(`http://localhost:${port}/Contract`, {
    method: "POST",
    body: JSON.stringify({
      ...contract,
      revisionId,
      source: sourceAddress,
      destination: destinationAddress,
    }),
    headers: {
      "Content-Type": "application/json",
    },
  });
}

export async function getContracts(port, contractFilter) {
  let result = await fetch(`http://localhost:${port}/GetContracts`, {
    method: "POST",

    body: JSON.stringify(contractFilter),
    headers: {
      "Content-Type": "application/json",
    },
  });
  const response = await result.json();
  return response.Contracts;
}

export async function createReceipt(port, contractId, type = receipt.type) {
  return fetch(`http://localhost:${port}/Receipt`, {
    method: "POST",
    body: JSON.stringify({
      ...receipt,
      contractId,
      type
    }),
    headers: {
      "Content-Type": "application/json",
    },
  });
}

export async function getReceipts(port, filter) {
  let result = await fetch(`http://localhost:${port}/GetReceipts`, {
    method: "POST",
    body: JSON.stringify(filter),
    headers: {
      "Content-Type": "application/json",
    },
  });
  const response = await result.json();
  console.log("RESPONSE")
  console.log(response.Receipts)
  return response.Receipts;
}

export async function createMessage(port, sourceAddress, destinationAddress) {
  return fetch(`http://localhost:${port}/Message`, {
    method: "POST",
    body: JSON.stringify({
      ...message,
      source: sourceAddress,
      destination: destinationAddress,
      messageTime: Date.now()
    }),
    headers: {
      "Content-Type": "application/json",
    },
  });
}

export async function getMessages(port, filter) {
  let result = await fetch(`http://localhost:${port}/GetMessages`, {
    method: "POST",

    body: JSON.stringify(filter),
    headers: {
      "Content-Type": "application/json",
    },
  });
  const response = await result.json();
  return response.Messages;
}
