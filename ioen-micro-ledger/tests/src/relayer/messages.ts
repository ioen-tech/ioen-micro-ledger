import { sleep } from "../utils";
import { spawnPlayers } from "./spawn";
import { createMessage, getMessages } from "./queries";

export default function test(orchestrator) {
  orchestrator.registerScenario(
    "create a message for an edge, and get it through the market",
    async (s, t) => {
      const {
        cloud,
        edges: [edge1, edge2],
      } = await spawnPlayers(s, 2);

      const before = Date.now() - 59000;
      await sleep(100);

      await createMessage(
        cloud.relayer.port,
        cloud.applicationId,
        edge1.applicationId
      );
      await sleep(2000);

      let messages = await getMessages(edge1.relayer.port, {});
      t.equals(messages.length, 1);

      messages = await getMessages(cloud.relayer.port, {});
      t.equals(messages.length, 1);

      await createMessage(
        cloud.relayer.port,
        cloud.applicationId,
        edge2.applicationId
      );
      await sleep(2000);
      messages = await getMessages(cloud.relayer.port, {});
      t.equals(messages.length, 2);

      messages = await getMessages(edge1.relayer.port, {});
      t.equals(messages.length, 1);

      // Test the filter by time range
      messages = await getMessages(cloud.relayer.port, {
        messageTime: Date.now(),
      });
      t.equals(messages.length, 2);

      messages = await getMessages(cloud.relayer.port, {
        messageTime: before,
      });
      t.equals(messages.length, 1);
      messages = await getMessages(cloud.relayer.port, {
        messageTime: before - 2000,
      });
      t.equals(messages.length, 0);

      cloud.relayer.process.kill();
      edge1.relayer.process.kill();
      edge2.relayer.process.kill();
    }
  );
}
