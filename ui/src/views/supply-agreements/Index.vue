<template>
  <section id="supply-agreements">
    <v-row no-gutters class="pa-2">
      <h1>Suppliers Agreements</h1>
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
      <v-col class="pa-2"
        cols="12">
        <v-btn
          dark
          @click.stop="newAgreementDialog = true">
          Create New Agreement
        </v-btn>
      </v-col>
    </v-row>
    <v-row>
    <v-dialog
      v-model="newAgreementDialog"
      transition="dialog-bottom-transition"
    >
      <v-card>
        <v-card-title>New Supply Agreement</v-card-title>
        <v-row no-gutters class="pa-2">
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
        <v-card-actions>
          <v-btn
            @click="createSupplyAgreement(agreement); newAgreementDialog = false"
            class="mr-2"
          >
            submit
          </v-btn>
          <v-btn @click.stop="newAgreementDialog = false">
            Cancel
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
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
    newAgreementDialog: false,
    agreement: {
      from: '1',
      to: '1',
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
