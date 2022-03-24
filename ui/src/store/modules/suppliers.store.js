import Vue from 'vue'
import Vuex from 'vuex'
import * as SuppliersApi from '@/api/suppliers'

Vue.use(Vuex)

export default {
  namespaced: true,
  state: {
    supplier: {}
  },
  actions: {
    initialise ({ commit }) {
      SuppliersApi.connect(process.env.VUE_APP_HC_PORT, () => {
        SuppliersApi.agentInfoSupplier((supplier) => {
          if (supplier[0] !== undefined) commit('createSupplier', supplier[0])
        })
      })
    },
    createSupplier ({ commit }, payload) {
      const supplier = payload
      commit('createSupplier', supplier)
      return new Promise((resolve, reject) => {
        SuppliersApi.createSupplier(supplier, (supplierHashes) => {
          commit('updateSupplier', supplierHashes)
          resolve(supplierHashes)
        })
      })
    }
  },
  mutations: {
    createSupplier (state, payload) {
      state.supplier = payload
    },
    updateSupplier (state, payload) {
      const supp = { ...state.supplier }
      supp.entryHash = payload.entry_hash
      supp.headerHash = payload.header_hash
      state.supplier = supp
    }
  },
  modules: {}
}
