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
    async initialise () {
      console.log(process.env)
      SuppliersApi.connect(process.env.VUE_APP_HC_PORT)
    },
    createSupplier ({ commit }, payload) {
      const supplier = payload
      commit('createSupplier', supplier)
      SuppliersApi.createSupplier(supplier, (committedSupplier) => {
        commit('updateSupplier', committedSupplier)
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
