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
    async initialise () {
      SupplyAgreementsApi.connect('5000')
    },
    createSupplyAgreement ({ commit }, payload) {
      const supplyAgreement = payload
      commit('createSupplyAgreement', supplier)
      SupplyAgreementsApi.createSupplyAgreement(supplyAgreement, (committedSupplyAgreement) => {
        commit('updateSupplyAgreement', committedSupplyAgreement)
      })
    },
    listSupplyAgreements ({ commit }) {
      SupplyAgreementsApi.listSupplyAgreements(result => {
        commit('setSupplyAgreements', result.supplyAgreements)
      })
    }
  },
  mutations: {
    setSupplyAgreements (state, payload) {
      state.suppliers = payload
    },
    createSupplyAgreement (state, payload) {
      state.suppliers.splice(0, 0, payload)
    },
    updateSupplyAgreement (state, payload) {
      state.supplyAgreements = state.supplyAgreements.map(supplier =>
        supplier.uuid !== payload.uuid ? supplier : { ...supplier, ...payload }
      )
    },
    deleteSupplyAgreement (state, payload) {
      state.supplyAgreements = state.supplyAgreements.filter(c => c.uuid !== payload.uuid)
    }
  },
  modules: {}
}
