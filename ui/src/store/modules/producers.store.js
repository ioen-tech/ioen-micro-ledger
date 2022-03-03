import Vue from 'vue'
import Vuex from 'vuex'
import * as ProducersApi from '@/api/producers'

Vue.use(Vuex)

export default {
  namespaced: true,
  state: {
    producers: []
  },
  actions: {
    async initialise ({ dispatch }) {
      ProducersApi.connect('5000')
    },
    createProducer ({ commit }, payload) {
      const producer = payload
      commit('createProducer', producer)
      ProducersApi.createProducer(producer, (committedProducer) => {
        console.log(committedProducer)
        commit('updateProducer', committedProducer)
      })
    },
    listProducers ({ state, commit }) {
      ProducersApi.listProducers(result => {
        commit('setProducers', result.producers)
      })
    }
  },
  mutations: {
    setProducers (state, payload) {
      state.producers = payload
    },
    createProducer (state, payload) {
      state.producers.splice(0, 0, payload)
    },
    updateProducer (state, payload) {
      state.producers = state.producers.map(producer =>
        producer.uuid !== payload.uuid ? producer : { ...producer, ...payload }
      )
    },
    deleteProducer (state, payload) {
      state.producers = state.producers.filter(c => c.uuid !== payload.uuid)
    }
  },
  modules: {}
}
