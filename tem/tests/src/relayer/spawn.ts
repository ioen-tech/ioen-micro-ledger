import path from "path";
import { fork } from "child_process";
import { sleep } from "../utils";
import getPort from "get-port";
import {
  Config,
  InstallAgentsHapps,
  Player,
  TransportConfigType,
  ProxyAcceptConfig,
  ProxyConfigType,
  NetworkType,
} from "@holochain/tryorama";
import Base64 from "js-base64";

const dnaTem = path.join(__dirname, "../../../dna/workdir/dna/tem.dna");

const network = {
  transport_pool: [
    {
      type: TransportConfigType.Quic,
    },
  ],
  bootstrap_service: "https://bootstrap-staging.holo.host",
  network_type: NetworkType.QuicBootstrap,
};

const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [dnaTem], // contains 1 dna, the "test" dna
  ],
];

export async function spawnPlayers(
  s,
  numberOfEdges: number,
  networking: boolean = true
) {
  const conductorConfig = Config.gen({
    network: networking ? network : undefined,
  });

  const [cloudPlayer]: Array<Player> = await s.players([conductorConfig]);

  const cloud = await createRelayer(cloudPlayer, "cloud-0", true);

  await sleep(2000);

  const edge_players: Array<Player> = networking
    ? await s.players(Array(numberOfEdges).fill(conductorConfig))
    : Array(numberOfEdges).fill(cloudPlayer);

  const edgePromises = edge_players.map(async (edge, i) =>
    createRelayer(edge, `edge-${i}`, false)
  );

  const edges = await Promise.all(edgePromises);
  // Wait for the bidders to retry to associate with the market if they need to
  await sleep(6000);

  return {
    cloud,
    edges,
  };
}

async function createRelayer(
  player: Player,
  applicationId: string,
  isCloud: boolean
) {
  const [[happ]] = await player.installAgentsHapps(installation);

  const edgeRelayer = await spawnRelayer(
    player,
    applicationId,
    happ.cells[0].cellId,
    isCloud
  );
  const agentAddress = serializeHash(happ.cells[0].cellId[1]);
  console.log(agentAddress);
  return {
    relayer: edgeRelayer,
    cell: happ.cells[0],
    applicationId,
  };
}

export function serializeHash(hash: Uint8Array): string {
  return `u${Base64.fromUint8Array(hash, true)}`;
}

export async function spawnRelayer(
  player: Player,
  applicationId: string,
  cellId,
  isCloud: boolean,
  debug = false
) {
  if (!player._conductor || !player._conductor.appClient) throw new Error("");

  const appPort = parseInt(
    (player._conductor.appClient.client as any).socket.url.split(":")[2]
  );

  const port = await getPort();
  const env = {
    CONDUCTOR_URL: `ws://localhost:${appPort}`,
    PORT: `${port}`,
    CELL_ID: JSON.stringify(cellId),
    APPLICATION_ID: applicationId,
  };

  if (debug) env["DEBUG"] = "true";
  if (isCloud) env["IS_CLOUD"] = "true";
  const process = fork(`../express-relayer/src/index.js`, [], {
    env,
    stdio: "inherit",
  });

  // Wait for the relayer to connect to the conductor
  await sleep(400);
  return { process, port };
}
