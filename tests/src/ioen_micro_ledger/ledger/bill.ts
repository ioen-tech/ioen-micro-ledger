
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  
  orchestrator.registerScenario("bill CRUD tests", async (s, t) => {
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

    const entryContents = {"address":"1 Ioen St","dueDate":1645151305198,"status":"unpaid","notes":""};

    // Alice creates a bill
    let create_output = await alice.call(
        "ledger",
        "create_bill",
        entryContents
    );
    t.ok(create_output.header_hash);
    t.ok(create_output.entry_hash);

    await sleep(50);
    
    // Bob gets the created bill
    let entry = await bob.call("ledger", "get_bill", create_output.entry_hash);
    t.deepEqual(entry, entryContents);
    
    
    // Alice updates the bill
    let update_output = await alice.call(
      "ledger",
      "update_bill",
      {
        original_header_hash: create_output.header_hash,
        updated_bill: {
          "notes": "nulla tempor occaecat",
  "dueDate": -61231305,
  "status": "commodo Excepteur",
  "address": "labore esse"
}
      }
    );
    t.ok(update_output.header_hash);
    t.ok(update_output.entry_hash);
    await sleep(50);

      
    
    // Alice delete the bill
    await alice.call(
      "ledger",
      "delete_bill",
      create_output.header_hash
    );
    await sleep(50);

    
    // Bob tries to get the deleted bill, but he doesn't get it because it has been deleted
    let deletedEntry = await bob.call("ledger", "get_bill", create_output.entry_hash);
    t.notOk(deletedEntry);
      
  });

}

