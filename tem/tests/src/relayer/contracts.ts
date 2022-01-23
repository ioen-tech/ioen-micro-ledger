import { sleep } from "../utils";
import { spawnPlayers } from "./spawn";
import {
  createContract,
  createReceipt,
  getContracts,
  getReceipts,
} from "./queries";

export default function test(orchestrator) {
  orchestrator.registerScenario(
    "create a contract for a bidder, and get it through the market",
    async (s, t) => {
      const {
        cloud,
        edges: [edge1, edge2],
      } = await spawnPlayers(s, 2);

      await createContract(
        cloud.relayer.port,
        1,
        cloud.applicationId,
        edge1.applicationId
      );
      await sleep(1000);

      let contracts = await getContracts(edge1.relayer.port, {});
      t.equals(contracts.length, 1);

      contracts = await getContracts(cloud.relayer.port, {});
      t.equals(contracts.length, 1);

      let receipts = await getReceipts(cloud.relayer.port, {});
      t.equals(receipts.length, 1);

      await createContract(
        cloud.relayer.port,
        2,
        cloud.applicationId,
        edge2.applicationId
      );
      await sleep(2000);
      contracts = await getContracts(cloud.relayer.port, {});
      t.equals(contracts.length, 2);

      contracts = await getContracts(edge1.relayer.port, {});
      t.equals(contracts.length, 1);

      contracts = await getContracts(cloud.relayer.port, {
        destination: edge2.applicationId,
      });
      t.equals(contracts.length, 1);

      // Trying to create a receipt for a non-existent contract
      const error = await createReceipt(edge1.relayer.port, 0);
      t.notOk(error.length);

      await createReceipt(edge1.relayer.port, 1);
      await createReceipt(edge2.relayer.port, 2, "type5");
      await sleep(2000);

      receipts = await getReceipts(cloud.relayer.port, {});
      t.equals(receipts.length, 4);

      receipts = await getReceipts(cloud.relayer.port, {
        type: "type5",
      });
      t.equals(receipts.length, 1);

      receipts = await getReceipts(edge1.relayer.port, {});
      t.equals(receipts.length, 2);

      cloud.relayer.process.kill();
      edge1.relayer.process.kill();
      edge2.relayer.process.kill();
    }
  );
}
