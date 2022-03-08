<template>
  <section id="consumer">
    <v-row no-gutters class="pa-2">
      <h1>Consumer</h1>
      <v-form v-model="valid">
        <v-container>
          <v-row>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="consumer.address"
                label="Address"
                required
              ></v-text-field>
            </v-col>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="consumer.postcode"
                :counter="4"
                label="Post Code"
                required
              ></v-text-field>
            </v-col>
          </v-row>
        </v-container>
        <v-spacer></v-spacer>
        <v-btn
          @click="createConsumer(consumer); $router.push('/generation-methods')"
          class="mr-2"
        >
          Submit
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
  name: 'Consumer',

  components: {
  },
  data: () => ({
    valid: false,
    consumer: {
      method: '',
      address: ''
    }
  }),
  methods: {
    ...mapActions('consumers', ['createConsumer'])
  },
  created () {
    this.$store.dispatch('consumers/initialise')
  }
}
</script>
