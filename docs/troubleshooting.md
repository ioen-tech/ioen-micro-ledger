# Troubleshooting the docker images

In order to see the logs of the running images, run these steps:

1. Find out the id of the container where the images are running with `docker ps`:

```
CONTAINER ID        IMAGE                                 COMMAND             CREATED             STATUS              PORTS                    NAMES
8c86a63b6fd9        guillemcordoba/monashtem-ledger:0.10   "sim2h_server"      10 minutes ago      Up 2 seconds        0.0.0.0:9000->9000/tcp   network_sim2h_server_1
```

2. Run `docker logs <CONTAINER_ID> --tail 200` to see the latest logs from that container. You can change the `200` to the number of lines you want to show.

This works for all images we are publishing to dockerhub. All images have attached to their logs the main service they are running, i.e. the conductor in the case of the holochain node, the node service in the case of the relayer, and the sim2h_server in the case of the sim2h_server container.