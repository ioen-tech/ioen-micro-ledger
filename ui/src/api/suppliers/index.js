import { AppWebsocket } from '@holochain/client'

// const signalCb = (signal) => {
//   resolve()
// }
let cellId = null
let hcClient = null

export function connect (port, callback) {
  AppWebsocket.connect(`ws://localhost:${port}`, 12000)
    .then(socket => {
      hcClient = socket
      socket.appInfo({
        installed_app_id: 'ioen-micro-ledger'
      })
        .then(appInfo => {
          cellId = appInfo.cell_data.find(data => data.role_id === 'ioen_micro_ledger').cell_id
          callback()
        })
    })
}

export function agentInfoSupplier (callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'agent_info_supplier',
    provenance: cellId[1],
    payload: null
  }).then(result => callback(result))
}

export function createSupplier (supplier, callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'create_supplier',
    provenance: cellId[1],
    payload: supplier
  }).then(supplierHashes => callback(supplierHashes))
}

export function deleteSupplier (supplier) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'delete_supplier',
    provenance: cellId[1],
    payload: supplier
  }).then(result => console.log(result))
}

export function listSuppliers (filter, callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'list_suppliers',
    provenance: cellId[1],
    payload: filter
  }).then(result => callback(result))
}
