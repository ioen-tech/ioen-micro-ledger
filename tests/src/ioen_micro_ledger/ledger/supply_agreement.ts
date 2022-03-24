
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  
  orchestrator.registerScenario("supply_agreement CRUD tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellRole.includes('/ioen_micro_ledger.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellRole.includes('/ioen_micro_ledger.dna')) as Cell;

    const supplierEntryContents = {"address":"123 Ioen St","postcode":"3149","method":"solar"};

    // Alice creates 2 suppliers
    let supplierEntry = await alice.call(
        "ledger",
        "create_supplier",
        supplierEntryContents
    );

    const entryContents = {"from":1645151305198,"to":1645151305198,"rate":0.1};

    let new_supply_agreement = {
      supplierEntryHash: supplierEntry.entry_hash,
      supplyAgreement: entryContents
    }
    // Alice creates a supply_agreement
    let create_output = await alice.call(
        "ledger",
        "create_supply_agreement",
        new_supply_agreement
    );
    t.ok(create_output.header_hash);
    t.ok(create_output.entry_hash);

    const entryContents2 = {"from":1645151305198,"to":1645151305198,"rate":0.1};

    let new_supply_agreement2 = {
      supplierEntryHash: supplierEntry.entry_hash,
      supplyAgreement: entryContents2
    }
    // Alice creates a supply_agreement
    let create_output2 = await alice.call(
      "ledger",
      "create_supply_agreement",
      new_supply_agreement2
    );
    t.ok(create_output2.header_hash);
    t.ok(create_output2.entry_hash);
    console.log(create_output2)
    await sleep(50);
    
    // Bob gets the created supply_agreement
    let entry = await bob.call("ledger", "get_supply_agreement", create_output.entry_hash);
    t.deepEqual(entry, entryContents);
    
    let existing_supply_agreement = {
      supplier_entry_hash: supplierEntry.entry_hash,
      supply_agreement_entry_hash: create_output.entry_hash
    }

    let link_to_exisiting_output = await alice.call("ledger", "existing_supply_agreement", existing_supply_agreement);
    t.ok(link_to_exisiting_output);
  });

}

