## Setting up a node

```bash
sh scripts/setup_node.sh [NODE_IP]
```

This should set everything up. To check if everything is working, do:

```bash
ssh -v -i IOENTest.tx ubuntu@[NODE_IP]
docker-compose up
```

## Update the scripts

To update the scripts for all of the nodes, you can use:

```bash
./scripts/update_scripts.sh
```

This will replace the `scripts` folder of each of the nodes, and also its `docker-compose.yml`


## Running the tests

To run a global network test, run:

```bash
./scripts/run_tests_in_all_nodes.sh [NUMBER_OF_AGENTS]
```

This will generate a new folder under the `testresults` folder, collecting the results from the nodes. The test results will be under a folder with the time that the test began, and the IP of each node. You will find a file named `system_usage.txt` (system usage watch during the test) and `test_results.txt` (output from `newman`).

While the test is running, the agents commit BidCommitments to itself and to the Market. The Market waits (with `sleep`) that all other agents have finished and then commits a MarketPlan and gets the bidcommitments, to check if the BidCommitments from the agents were also stored on the Market.

## Changing the delay between iterations

1. `nano scripts/run_tests.sh`
2. Change the --delay-request parameter (it's in ms)
3. Run `./scripts/update_scripts.sh`
4. Ready to run tests

## Troubleshooting

If you see an error like "Address already in use", "cannot bind to interface", etc., running this might solve it:

```bash
docker network disconnect -f ubuntu_default ubuntu_express1_1
```