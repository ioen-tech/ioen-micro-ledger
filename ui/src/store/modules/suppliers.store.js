import Vue from 'vue'
import Vuex from 'vuex'
import * as ProducersApi from '@/api/suppliers'

Vue.use(Vuex)

export default {
  namespaced: true,
  state: {
    suppliers: []
  },
  actions: {
    async initialise ({ dispatch }) {
      ProducersApi.connect('5000')
    },
    createProducer ({ commit }, payload) {
      const supplier = payload
      commit('createProducer', supplier)
      ProducersApi.createProducer(supplier, (committedProducer) => {
        console.log(committedProducer)
        commit('updateProducer', committedProducer)
      })
    },
    listProducers ({ state, commit }) {
      ProducersApi.listProducers(result => {
        commit('setProducers', result.suppliers)
      })
    }
  },
  mutations: {
    setProducers (state, payload) {
      state.suppliers = payload
    },
    createProducer (state, payload) {
      state.suppliers.splice(0, 0, payload)
    },
    updateProducer (state, payload) {
      state.suppliers = state.suppliers.map(supplier =>
        supplier.uuid !== payload.uuid ? supplier : { ...supplier, ...payload }
      )
    },
    deleteProducer (state, payload) {
      state.suppliers = state.suppliers.filter(c => c.uuid !== payload.uuid)
    }
  },
  modules: {}
}
