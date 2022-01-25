# Specification for ioen-ledger 

## Description
The ioen-ledger is the most basic component in the IOEN architecture; it communicates to the peer-to-peer distributed framework, Holochain.  It is associated
with one agent.  Functions:
- reads its agent's ID
- sends a peer-to-peer message to another agent ID from this agent
- provides a REST API to write a record.  The record format is undefined, which allows any schema to be accommodated.  The agent ID is required.
- provides the API to fetch one or more records from one agent.  The records are retrieved based on the agent ID, and allow a timeframe
-- `give me all records on this agent between timestamp m and timestamp n, where m > n`
- performance tests
- scripts to spin up virtual machines based on IP address
- creation of a Docker container

## Technology
This includes:
- Holochain core, in Rust.  Some uses may not need Rust expertise
- Express-relayer, the API, is written in Javascript

## Uses
- None. This is a core component.

## Used by
- https://github.com/ioen-tech/ioen-discovery
- https://github.com/ioen-tech/ioen-ledger-report-generator

## This version
0.1 is very primitive (it has been adapted from a commercial project so functionality has been removed).  Future functionality will include:
- Discovery of other agents
- A basic set of validation rules
- An enhanced set of validation rules that represent the microDAO (applied for all agents operating in the DHT)

## Other functionality elsewhere
- Holochain rules are encapsulated in the DNA.  IOEN will provide a tool to allow these to be specified and shared in another component.
