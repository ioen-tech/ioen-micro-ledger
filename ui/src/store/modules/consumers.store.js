import Vue from 'vue'
import Vuex from 'vuex'
import * as ConsumersApi from '@/api/consumers'

Vue.use(Vuex)

export default {
  namespaced: true,
  state: {
    consumer: {}
  },
  actions: {
    async initialise ({ state, commit }) {
      ConsumersApi.connect(process.env.VUE_APP_HC_PORT, () => {
        ConsumersApi.agentInfoConsumer((consumer) => {
          if (consumer[0] !== undefined) {
            commit('createConsumer', consumer[0])
          }
        })
      })
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
    createConsumer (state, payload) {
      state.consumer = payload
    },
    updateConsumer (state, payload) {
      const cons = { ...state.consumer }
      cons.entryHash = payload.entry_hash
      cons.headerHash = payload.header_hash
      state.consumer = cons
    },
    deleteConsumer (state, payload) {
      state.consumers = state.consumers.filter(c => c.uuid !== payload.uuid)
    }
  },
  modules: {}
}
