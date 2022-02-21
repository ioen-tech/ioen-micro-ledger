
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  
  orchestrator.registerScenario("producer CRUD tests", async (s, t) => {
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

    const entryContents = {"address":"123 Ioen St","postcode":3149,"method":"solar"};

    // Alice creates a producer
    let create_output = await alice.call(
        "ledger",
        "create_producer",
        entryContents
    );
    t.ok(create_output.header_hash);
    t.ok(create_output.entry_hash);

    await sleep(50);
    
    // Bob gets the created producer
    let entry = await bob.call("ledger", "get_producer", create_output.entry_hash);
    t.deepEqual(entry, entryContents);
    
    
    // Alice updates the producer
    let update_output = await alice.call(
      "ledger",
      "update_producer",
      {
        original_header_hash: create_output.header_hash,
        updated_producer: {
          "method": "incididunt laborum tempor",
  "postcode": -11958844,
  "address": "aliqua ad qui in anim"
}
      }
    );
    t.ok(update_output.header_hash);
    t.ok(update_output.entry_hash);
    await sleep(50);

      
    
    // Alice delete the producer
    await alice.call(
      "ledger",
      "delete_producer",
      create_output.header_hash
    );
    await sleep(50);

    
    // Bob tries to get the deleted producer, but he doesn't get it because it has been deleted
    let deletedEntry = await bob.call("ledger", "get_producer", create_output.entry_hash);
    t.notOk(deletedEntry);
      
  });

}

