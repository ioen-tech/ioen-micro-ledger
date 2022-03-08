import Vue from 'vue'
import Vuex from 'vuex'
import * as SuppliersApi from '@/api/suppliers'

Vue.use(Vuex)

export default {
  namespaced: true,
  state: {
    suppliers: []
  },
  actions: {
    async initialise () {
      SuppliersApi.connect('5000')
    },
    createSupplier ({ commit }, payload) {
      const supplier = payload
      commit('createSupplier', supplier)
      SuppliersApi.createSupplier(supplier, (committedSupplier) => {
        console.log(committedSupplier)
        commit('updateSupplier', committedSupplier)
      })
    },
    listSuppliers ({ commit }) {
      SuppliersApi.listSuppliers(result => {
        commit('setSuppliers', result.suppliers)
      })
    }
  },
  mutations: {
    setSuppliers (state, payload) {
      state.suppliers = payload
    },
    createSupplier (state, payload) {
      state.suppliers.splice(0, 0, payload)
    },
    updateSupplier (state, payload) {
      state.suppliers = state.suppliers.map(supplier =>
        supplier.uuid !== payload.uuid ? supplier : { ...supplier, ...payload }
      )
    },
    deleteSupplier (state, payload) {
      state.suppliers = state.suppliers.filter(c => c.uuid !== payload.uuid)
    }
  },
  modules: {}
}
