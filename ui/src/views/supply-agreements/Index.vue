<template>
  <section id="supply-agreements">
    <v-row no-gutters class="pa-2">
      <h1>New Supply Agreement</h1>
      <v-form v-model="valid">
        <v-container>
          <v-row>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="supplier.from"
                label="From"
                required
              ></v-text-field>
            </v-col>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="supplier.to"
                label="To"
                required
              ></v-text-field>
            </v-col>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="supplier.rate"
                label="Rate"
                required
              ></v-text-field>
            </v-col>
          </v-row>
        </v-container>
        <v-spacer></v-spacer>
        <v-btn
          @click="createS(agreement)"
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
import { mapActions } from 'vuex'

export default {
  name: 'SupplyAgreements',

  components: {
  },
  data: () => ({
    valid: false,
    agreement: {
      from: '',
      to: '',
      rate: ''
    }
  }),
  methods: {
    ...mapActions('suppliers', ['createS'])
  },
  created () {
    this.$store.dispatch('suppliers/initialise')
  }
}
</script>
