import { Orchestrator } from "@holochain/tryorama";
import messages from "./relayer/messages";

const orchestrator = new Orchestrator();
messages(orchestrator);
orchestrator.run();
