<template>
  <section id="demo-setup">
    <v-container>
      <v-row>
        <v-col
          cols="12"
          md="4"
          v-if="consumer.address === undefined"
        >
          <v-card
            max-width="400"
          >
            <v-toolbar
              color="green"
              dark
            >
              <v-toolbar-title>Consumer Demo Options</v-toolbar-title>
            </v-toolbar>

            <v-list
              subheader
              two-line
            >
              <v-list-item
                v-for="demoConsumer in demoConsumers"
                :key="demoConsumer.address"
              >
                <v-list-item-avatar>
                  <v-icon
                    class="grey lighten-1"
                    dark
                  >
                    mdi-account
                  </v-icon>
                </v-list-item-avatar>
                <v-list-item-content>
                  <v-list-item-title v-text="demoConsumer.address"></v-list-item-title>

                  <v-list-item-subtitle v-text="demoConsumer.postcode"></v-list-item-subtitle>
                  <v-list-item-subtitle v-text="demoConsumer.description"></v-list-item-subtitle>
                </v-list-item-content>
                <v-list-item-action>
                  <v-btn
                    icon
                    @click="setupConsumer(demoConsumer)">
                    <v-icon color="action">mdi-arrow-right-circle</v-icon>
                  </v-btn>
                </v-list-item-action>
              </v-list-item>
            </v-list>
          </v-card>
        </v-col>
        <v-col
          cols="12"
          md="4"
          v-if="supplier.address === undefined"
        >
          <v-card
            max-width="400"
          >
            <v-toolbar
              color="purple"
              dark
            >
              <v-toolbar-title>Supplier Demo Options</v-toolbar-title>
            </v-toolbar>

            <v-list
              subheader
              two-line
            >
              <v-list-item
                v-for="demoSupplier in demoSuppliers"
                :key="demoSupplier.address"
              >
                <v-list-item-avatar>
                  <v-icon
                    class="grey lighten-1"
                    dark
                  >
                    mdi-account
                  </v-icon>
                </v-list-item-avatar>

                <v-list-item-content>
                  <v-list-item-title v-text="demoSupplier.address"></v-list-item-title>

                  <v-list-item-subtitle v-text="demoSupplier.postcode"></v-list-item-subtitle>
                  <v-list-item-subtitle v-text="demoSupplier.description"></v-list-item-subtitle>
                </v-list-item-content>

                <v-list-item-action>
                  <v-btn
                    icon
                    @click="setupSupplier(demoSupplier)">
                    <v-icon color="action">mdi-arrow-right-circle</v-icon>
                  </v-btn>
                </v-list-item-action>
              </v-list-item>
            </v-list>
          </v-card>
        </v-col>
      </v-row>
    </v-container>
  </section>
</template>

<script>
import { mapActions, mapState } from 'vuex'
const contractTime = 24 * 60 * 60 * 1000

export default {
  name: 'Consumer',

  components: {
  },
  data: () => ({
    demoConsumers: [
      {
        method: 'solar',
        address: '123 Ioen Rd',
        postcode: '3000',
        description: 'Consumer entry only'
      },
      {
        method: 'solar',
        address: '123 Sesame St',
        postcode: '3001',
        description: 'Consumer with open negotiation'
      },
      {
        method: 'wind',
        address: '33 Redgrid Drive',
        postcode: '3000',
        description: 'Consumer with a negotiated supply agreement'
      },
      {
        method: 'battery',
        address: '420 Solar St',
        postcode: '3000',
        description: 'Consumer with Bill'
      },
      {
        method: 'solar',
        address: '27 Alice Drive',
        postcode: '3001',
        description: 'Consumer with a negotiated supply agreement'
      },
      {
        method: 'wind',
        address: '12 High Street',
        postcode: '3001',
        description: 'Consumer with open negotiation'
      },
      {
        method: 'wind',
        address: '45 Sunhill St',
        postcode: '3001',
        description: 'Consumer with Bill'
      },
      {
        method: 'wind',
        address: '6 Fire Drive',
        postcode: '3001',
        description: 'Consumer with a negotiated supply agreement'
      }
    ],
    demoSuppliers: [
      {
        method: 'solar',
        address: '1 Ioen-Supplier Rd',
        postcode: '3000',
        description: 'Supplier entry only',
        agreements: []
      },
      {
        method: 'wind',
        address: '3 Redgrid-Supplier Drive',
        postcode: '3000',
        description: 'Supplier with created agreements',
        agreements: [
          {
            from: Date.now(),
            to: Date.now() + contractTime,
            rate: 2
          },
          {
            from: Date.now(),
            to: Date.now() + contractTime,
            rate: 20
          }
        ]
      },
      {
        method: 'battery',
        address: '4 Solar-Supplier St',
        postcode: '3000',
        description: 'Supplier with open negotiation',
        agreements: []
      },
      {
        method: 'solar',
        address: '2 Sesame-Supplier St',
        postcode: '3001',
        description: 'Bill with SupplyBlocks',
        agreements: []
      }
    ]
  }),
  computed: {
    ...mapState('suppliers', ['supplier']),
    ...mapState('consumers', ['consumer'])
  },
  methods: {
    ...mapActions('consumers', ['createConsumer']),
    ...mapActions('suppliers', ['createSupplier']),
    ...mapActions('supply-agreements', ['createSupplyAgreement']),
    setupSupplier (demoSupplier) {
      this.createSupplier(demoSupplier).then(committedSupplier => {
        console.log(committedSupplier)
        let index = 1
        demoSupplier.agreements.forEach(agreement => {
          console.log('supply-agreements', index)
          setTimeout(() => {
            this.createSupplyAgreement(agreement).then(supplyAgreementHashes => {
              console.log(supplyAgreementHashes)
            })
          }, index * 1000)
          index = index + 1
        })
      })
      this.$router.push('/profile')
    },
    setupConsumer (consumer) {
      this.createConsumer(consumer)
      this.$router.push('/profile')
    }
  },
  created () {
    this.$store.dispatch('consumers/initialise')
    this.$store.dispatch('suppliers/initialise')
  }
}
</script>
