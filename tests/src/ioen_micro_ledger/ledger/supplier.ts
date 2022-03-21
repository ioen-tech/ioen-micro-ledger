
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  
  orchestrator.registerScenario("supplier CRUD tests", async (s, t) => {
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

    const entryContents = {"address":"123 Ioen St","postcode":"3149","method":"solar"};
    const entry2Contents = {"address":"1 Redgrid St","postcode":"3149","method":"solar"};

    // Alice creates 2 suppliers
    let create_output = await alice.call(
        "ledger",
        "create_supplier",
        entryContents
    );
    t.ok(create_output.header_hash);
    t.ok(create_output.entry_hash);
    await bob.call(
      "ledger",
      "create_supplier",
      entry2Contents
    );
    await sleep(50);

    let aliceAgentinfoSupplier = await alice.call(
      "ledger",
      "agent_info_supplier"
    );
    t.deepEqual(aliceAgentinfoSupplier[0].address, "123 Ioen St");

    let bobAgentinfoSupplier = await bob.call(
      "ledger",
      "agent_info_supplier"
    );
    t.deepEqual(bobAgentinfoSupplier[0].address, "1 Redgrid St");

    // Bob gets the first created supplier
    let entry = await bob.call("ledger", "get_supplier", create_output.entry_hash);
    t.deepEqual(entry, entryContents);
    
     // Alice lists the suppliers in solar 3149
    let list_output = await alice.call(
      "ledger",
      "list_suppliers",
      {
        "method": "solar",
        "postcode": "3149",
      }
    );
    t.ok(list_output.length === 2)

    // Alice updates the supplier
    let update_output = await alice.call(
      "ledger",
      "update_supplier",
      {
        original_header_hash: create_output.header_hash,
        updated_supplier: {
          "method": "incididunt laborum tempor",
          "postcode": "444",
          "address": "aliqua ad qui in anim"
      }
    });
    t.ok(update_output.header_hash);
    t.ok(update_output.entry_hash);
    await sleep(50);

      
    
    // Alice delete the supplier
    await alice.call(
      "ledger",
      "delete_supplier",
      create_output.header_hash
    );
    await sleep(50);

    
    // Bob tries to get the deleted supplier, but he doesn't get it because it has been deleted
    let deletedEntry = await bob.call("ledger", "get_supplier", create_output.entry_hash);
    t.notOk(deletedEntry);
      
  });

}

