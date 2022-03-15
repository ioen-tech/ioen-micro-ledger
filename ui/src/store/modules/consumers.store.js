import Vue from 'vue'
import Vuex from 'vuex'
import * as ConsumersApi from '@/api/consumers'

Vue.use(Vuex)

export default {
  namespaced: true,
  state: {
    consumers: []
  },
  actions: {
    async initialise ({ dispatch }) {
      ConsumersApi.connect(process.env.VUE_APP_HC_PORT)
    },
    createConsumer ({ commit }, payload) {
      const consumer = payload
      commit('createConsumer', consumer)
      ConsumersApi.createConsumer(consumer, (committedConsumer) => {
        console.log(committedConsumer)
        commit('updateConsumer', committedConsumer)
      })
    },
    listConsumers ({ state, commit }) {
      ConsumersApi.listConsumers(result => {
        commit('setConsumers', result.consumers)
      })
    }
  },
  mutations: {
    setConsumers (state, payload) {
      state.consumers = payload
    },
    createConsumer (state, payload) {
      state.consumers.splice(0, 0, payload)
    },
    updateConsumer (state, payload) {
      state.consumers = state.consumers.map(consumer =>
        consumer.uuid !== payload.uuid ? consumer : { ...consumer, ...payload }
      )
    },
    deleteConsumer (state, payload) {
      state.consumers = state.consumers.filter(c => c.uuid !== payload.uuid)
    }
  },
  modules: {}
}
