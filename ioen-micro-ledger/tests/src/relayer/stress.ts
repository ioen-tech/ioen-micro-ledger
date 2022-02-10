import * as _ from "lodash";
import { sleep } from "../utils";
import { createContract } from "./queries";
import { spawnPlayers } from "./spawn";
import fetch from "node-fetch";

const NUMBER_OF_EDGES = 30;
const MS_BETWEEN_COMMITS = 1000;
const NUMBER_OF_ENTRIES = 10;

async function stressCloud(t, cloud, edges, numberOfEntries) {
  const startTime = Date.now();

  for (let i = 0; i < numberOfEntries; i++) {
    let edgeIndex = Math.floor(Math.random() * 100) % edges.length;
    await createContract(
      cloud.relayer.port,
      i,
      cloud.agentAddress,
      edges[edgeIndex].agentAddress
    );
    await sleep(MS_BETWEEN_COMMITS);
  }

  const result = await fetch(`http://localhost:${cloud.relayer.port}/Contract`);

  console.log(
    `Time for ${cloud.relayer.port}: ${(Date.now() - startTime) / 1000}`
  );

  const response = await result.json();
  t.equals(response.Contracts.length, numberOfEntries);
}

async function stressEdge(t, edge, cloud, numberOfEntries) {
  const startTime = Date.now();

  for (let i = 0; i < numberOfEntries; i++) {
    await createContract(
      edge.relayer.port,
      i,
      edge.agentAddress,
      cloud.agentAddress
    );
    await sleep(MS_BETWEEN_COMMITS);
  }

  const result = await fetch(`http://localhost:${edge.relayer.port}/Contract`);

  console.log(
    `Time for ${edge.relayer.port}: ${(Date.now() - startTime) / 1000}`
  );

  const response = await result.json();
  //t.equals(response.BidCommitments.length, numberOfEntries);
}

export default function test(orchestrator) {
  orchestrator.registerScenario("stress a number of players", async (s, t) => {
    // spawn the conductor process
    const { cloud, edges } = await spawnPlayers(s, NUMBER_OF_EDGES);

    await sleep(NUMBER_OF_EDGES * 800);

    let cloudStress = stressCloud(t, cloud, edges, NUMBER_OF_ENTRIES);

    let edgesStress = edges.map((edge) =>
      stressEdge(t, edge, cloud, NUMBER_OF_ENTRIES)
    );
    await sleep(50);

    await Promise.all([cloudStress, ...edgesStress]);

    cloud.relayer.process.kill();
    edges.forEach((edge) => edge.relayer.process.kill());
  });
}
