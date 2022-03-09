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
          callback(cellId)
        })
    })
}

export function createSupplyAgreement (supplyAgreement, callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'create_supply_agreement',
    provenance: cellId[1],
    payload: supplyAgreement
  }).then(committedSupplyAgreement => callback(committedSupplyAgreement))
}

export function deleteSupplyAgreement (supplyAgreement) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'delete_supply_agreement',
    provenance: cellId[1],
    payload: supplyAgreement
  }).then(result => console.log(result))
}

export function listSuppliersAgreements (supplier, callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'list_suppliers_agreements',
    provenance: cellId[1],
    payload: supplier
  }).then(result => callback(result))
}

export function listAllSupplyAgreements (callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'list_all_supply_agreements',
    provenance: cellId[1]
  }).then(result => callback(result))
}
