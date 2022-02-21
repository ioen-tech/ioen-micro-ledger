
import { Orchestrator } from "@holochain/tryorama";

import ledger_consumer from './ioen_micro_ledger/ledger/consumer';
import ledger_producer from './ioen_micro_ledger/ledger/producer';
import ledger_supply_agreement from './ioen_micro_ledger/ledger/supply_agreement';
import ledger_supply_block from './ioen_micro_ledger/ledger/supply_block';
import ledger_bill from './ioen_micro_ledger/ledger/bill';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
ledger_consumer(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
ledger_producer(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
ledger_supply_agreement(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
ledger_supply_block(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
ledger_bill(orchestrator);
orchestrator.run();



