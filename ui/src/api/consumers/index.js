import { AppWebsocket } from '@holochain/client'

// const signalCb = (signal) => {
//   resolve()
// }
let cellId = null
let hcClient = null

export function connect (port) {
  AppWebsocket.connect(`ws://localhost:${port}`, 12000)
    .then(socket => {
      hcClient = socket
      socket.appInfo({
        installed_app_id: 'ioen-micro-ledger'
      })
        .then(appInfo => {
          cellId = appInfo.cell_data.find(data => data.role_id === 'ioen_micro_ledger').cell_id
        })
    })
}

export function createConsumer (consumer, callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'create_consumer',
    provenance: cellId[1],
    payload: consumer
  }).then(committedConsumer => callback(committedConsumer))
}

export function deleteConsumer (consumer) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'delete_consumer',
    provenance: cellId[1],
    payload: consumer
  }).then(result => console.log(result))
}

export function listConsumers (filter, callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'list_consumers',
    provenance: cellId[1],
    payload: filter
  }).then(result => callback(result))
}
