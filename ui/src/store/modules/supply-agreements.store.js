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
      SupplyAgreementsApi.connect(process.env.VUE_APP_HC_PORT, (cellId) => {
        dispatch('listSuppliersAgreements')
      })
    },
    async createSupplyAgreement ({ rootState, commit }, payload) {
      return new Promise((resolve) => {
        const agreement = {
          from: parseInt(payload.from),
          to: parseInt(payload.to),
          rate: parseInt(payload.rate)
        }
        const newSupplyAgreement = {
          supplierEntryHash: rootState.suppliers.supplier.entryHash,
          supplyAgreement: agreement
        }
        SupplyAgreementsApi.createSupplyAgreement(newSupplyAgreement, (supplyAgreementHashes) => {
          commit('createSupplyAgreement', supplyAgreementHashes)
          resolve(supplyAgreementHashes)
        })
      })
    },
    listSupplyAgreements ({ commit }) {
      SupplyAgreementsApi.listAllSupplyAgreements(result => {
        console.log(result)
        commit('setAllSupplyAgreements', result)
      })
    },
    listSuppliersAgreements ({ rootState, commit }) {
      SupplyAgreementsApi.listSuppliersAgreements(rootState.suppliers.supplier.entryHash, result => {
        console.log(result)
        commit('setSupplyAgreements', result)
      })
    }
  },
  mutations: {
    setAllSupplyAgreements (state, payload) {
      state.supplyAgreements = payload
    },
    createSupplyAgreement (state, payload) {
      state.supplyAgreements.push(payload)
    },
    updateSupplyAgreement (state, payload) {
      state.supplyAgreements = state.supplyAgreements.map(supplyAgreement =>
        supplyAgreement.headerHash !== payload.headerHash ? supplyAgreement : { ...supplyAgreement, ...payload }
      )
    },
    deleteSupplyAgreement (state, payload) {
      state.supplyAgreements = state.supplyAgreements.filter(c => c.headerHash !== payload.headerHash)
    }
  },
  modules: {}
}
