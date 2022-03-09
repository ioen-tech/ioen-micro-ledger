import Vue from 'vue'
import Vuex from 'vuex'
import * as SupplyAgreementsApi from '@/api/supply-agreements'

Vue.use(Vuex)

export default {
  namespaced: true,
  state: {
    supplyAgreements: []
  },
  actions: {
    async initialise ({ dispatch }) {
      SupplyAgreementsApi.connect('5000', (cellId) => {
        dispatch('listSupplyAgreements')
      })
    },
    createSupplyAgreement ({ rootState, commit }, payload) {
      const agreement = {
        from: parseInt(payload.from),
        to: parseInt(payload.to),
        rate: parseInt(payload.rate)
      }
      const newSupplyAgreement = {
        supplier_entry_hash: rootState.suppliers.supplier.entryHash,
        supply_agreement: agreement
      }
      commit('createSupplyAgreement', payload)
      SupplyAgreementsApi.createSupplyAgreement(newSupplyAgreement, (committedSupplyAgreement) => {
        commit('updateSupplyAgreement', committedSupplyAgreement)
      })
    },
    listSupplyAgreements ({ commit }) {
      SupplyAgreementsApi.listAllSupplyAgreements(result => {
        console.log(result)
        commit('setSupplyAgreements', result)
      })
    }
  },
  mutations: {
    setSupplyAgreements (state, payload) {
      state.supplyAgreements = payload
    },
    createSupplyAgreement (state, payload) {
      state.supplyAgreements.splice(0, 0, payload)
    },
    updateSupplyAgreement (state, payload) {
      state.supplyAgreements = state.supplyAgreements.map(supplyAgreement =>
        supplyAgreement.uuid !== payload.uuid ? supplyAgreement : { ...supplyAgreement, ...payload }
      )
    },
    deleteSupplyAgreement (state, payload) {
      state.supplyAgreements = state.supplyAgreements.filter(c => c.uuid !== payload.uuid)
    }
  },
  modules: {}
}
