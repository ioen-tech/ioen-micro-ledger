// import { Orchestrator } from "@holochain/tryorama";
// import contracts from "./relayer/contracts";
// import messages from "./relayer/messages";
// import stress from "./relayer/stress";

// const orchestrator = new Orchestrator();

// if (process.env.STRESS) {
//   stress(orchestrator);
// } else {
//  messages(orchestrator);
//   // contracts(orchestrator);
// }

// orchestrator.run();

import { Orchestrator, Config, InstallAgentsHapps } from "@holochain/tryorama";
import path from "path";
import { createMessage, getMessages } from "./relayer/queries";

const conductorConfig = Config.gen();

// Construct proper paths for your DNAs
const demoDnaPath = path.join(__dirname, "../../dna/workdir/dna/tem.dna");

// create an InstallAgentsHapps array with your DNAs to tell tryorama what
// to install into the conductor.
const installation: InstallAgentsHapps = [
  // agent 0
  [
    // happ 0
    [demoDnaPath],
  ],
];

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(() => resolve(null), ms));

const orchestrator = new Orchestrator();

orchestrator.registerScenario("sample test", async (s, t) => {
  const [alice] = await s.players([conductorConfig]);

  // install your happs into the coductors and destructuring the returned happ data using the same
  // array structure as you created in your installation array.
  const [[alice_common]] = await alice.installAgentsHapps(installation);

  let message = {
    message_id: 1,
    contract_id: 1,

    source: "source",
    destination: "ApplicationId",

    type: "String",

    ts_milliseconds: 1000,

    payload: "test",
  }

  let result = alice_common.cells[0].call("tem", "create_message", message);
  
});

orchestrator.run();
