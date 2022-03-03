<template>
  <section id="producer">
    <v-row no-gutters class="pa-2">
      <h1>Producer</h1>
      <v-form v-model="valid">
        <v-container>
          <v-row>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="producer.address"
                label="Address"
                required
              ></v-text-field>
            </v-col>
            <v-col
              cols="12"
              md="4"
            >
              <v-text-field
                v-model="producer.postcode"
                :counter="4"
                label="Post Code"
                required
              ></v-text-field>
            </v-col>
            <v-col
              cols="12"
              md="4"
            >
              <v-select
                v-model="producer.method"
                :items="methods"
                label="Generation Method"
              ></v-select>
            </v-col>
          </v-row>
        </v-container>
        <v-spacer></v-spacer>
        <v-btn
          @click="createProducer(producer)"
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
  name: 'Producer',

  components: {
  },
  data: () => ({
    valid: false,
    methods: ['solar', 'wind', 'hydro', 'geo'],
    producer: {
      method: '',
      address: '',
      postcode: ''
    }
  }),
  methods: {
    ...mapActions('producers', ['createProducer'])
  },
  created () {
    this.$store.dispatch('producers/initialise')
  }
}
</script>
