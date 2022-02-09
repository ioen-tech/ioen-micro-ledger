import { Orchestrator } from "@holochain/tryorama";
import contracts from "./relayer/contracts";
import messages from "./relayer/messages";
import stress from "./relayer/stress";

const orchestrator = new Orchestrator();

if (process.env.STRESS) {
  stress(orchestrator);
} else {
  // contracts(orchestrator);
  messages(orchestrator);
}

orchestrator.run();
