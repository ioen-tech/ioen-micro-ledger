<template>
  <section id="supply-agreements">
    <v-row no-gutters class="pa-2">
      <h1>All Suppliers Agreements</h1>
      <v-col class="pa-2"
        cols="12"
        v-if="supplyAgreements.length === 0">
        <h3>No Supply Agreements yet.</h3>
      </v-col>
      <v-col class="pa-2"
        cols="12"
        v-else
        v-for="(supplyAgreement, index) in supplyAgreements"
        :key="index">
        <supply-agreement :agreement="supplyAgreement" />
      </v-col>
    </v-row>
    <v-row no-gutters class="pa-2">
      <h1>New Supply Agreement</h1>
      <v-form v-model="valid">
        <v-container>
          <v-row no-gutters>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="agreement.from"
                label="From"
                required
              ></v-text-field>
            </v-col>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="agreement.to"
                label="To"
                required
              ></v-text-field>
            </v-col>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="agreement.rate"
                label="Rate"
                required
              ></v-text-field>
            </v-col>
          </v-row>
        </v-container>
        <v-spacer></v-spacer>
        <v-btn
          @click="createSupplyAgreement(agreement)"
          class="mr-2"
        >
          submit
        </v-btn>
        <v-btn to="/">
          Cancel
        </v-btn>
      </v-form>
    </v-row>
  </section>
</template>

<script>
import { mapActions, mapState } from 'vuex'
import SupplyAgreement from '../../components/SupplyAgreement.vue'

export default {
  name: 'SupplyAgreements',

  components: {
    SupplyAgreement
  },
  data: () => ({
    valid: false,
    agreement: {
      from: '',
      to: '',
      rate: ''
    }
  }),
  computed: {
    ...mapState('supply-agreements', ['supplyAgreements'])
  },
  methods: {
    ...mapActions('supply-agreements', ['createSupplyAgreement'])
  },
  created () {
    this.$store.dispatch('supply-agreements/initialise')
  }
}
</script>
