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

export function listSupplyAgreements (filter, callback) {
  hcClient.callZome({
    cap: null,
    cell_id: cellId,
    zome_name: 'ledger',
    fn_name: 'list_supply_agreements',
    provenance: cellId[1],
    payload: filter
  }).then(result => callback(result))
}
