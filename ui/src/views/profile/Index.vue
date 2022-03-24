<template>
<section id="home">
    <v-row no-gutters>
      <v-img
        :min-height="'calc(100vh - ' + $vuetify.application.top + 'px)'"
        :src="require('@/assets/Home.png')"
      >
        <v-theme-provider dark>
          <v-container fill-height>
            <v-row
              align="start"
              class="mx-auto"
              justify="center"
            >
              <v-card
                width="300"
                color="grey"
                class="ma-2"
                v-if="supplier.address !== undefined"
              >
              <v-card-title>Supplier</v-card-title>
                <v-card-text>
                  <v-list
                    subheader
                    two-line
                  >
                    <v-list-item>
                      <v-list-item-content>
                        <v-list-item-subtitle v-text="supplier.address"></v-list-item-subtitle>
                        <v-list-item-subtitle v-text="supplier.postcode"></v-list-item-subtitle>
                        <v-list-item-subtitle v-text="supplier.method"></v-list-item-subtitle>
                      </v-list-item-content>
                    </v-list-item>
                  </v-list>
                </v-card-text>
              </v-card>
              <v-card
                width="300"
                color="grey"
                class="ma-2"
                v-for="supplyAgreement in supplyAgreements"
                :key="supplyAgreement.header_hash"
              >
              <v-card-title>Supply Agreement</v-card-title>
                <v-card-text>
                  <v-list
                    subheader
                    two-line
                  >
                    <v-list-item>
                      <v-list-item-content>
                        <v-list-item-subtitle v-text="supplyAgreement.from"></v-list-item-subtitle>
                        <v-list-item-subtitle v-text="supplyAgreement.to"></v-list-item-subtitle>
                        <v-list-item-subtitle v-text="supplyAgreement.rate"></v-list-item-subtitle>
                      </v-list-item-content>
                    </v-list-item>
                  </v-list>
                </v-card-text>
              </v-card>
              <v-card
                width="300"
                color="grey"
                class="ma-2"
                v-if="consumer.address !== undefined">
                <v-card-title>Consumer</v-card-title>
                  <v-card-text>
                    <v-list
                      subheader
                      two-line
                    >
                      <v-list-item>
                        <v-list-item-content>
                          <v-list-item-subtitle v-text="consumer.address"></v-list-item-subtitle>
                          <v-list-item-subtitle v-text="consumer.postcode"></v-list-item-subtitle>
                          <v-list-item-subtitle v-text="consumer.method"></v-list-item-subtitle>
                        </v-list-item-content>
                      </v-list-item>
                    </v-list>
                  </v-card-text>
              </v-card>
            </v-row>
          </v-container>
        </v-theme-provider>
      </v-img>
    </v-row>
  </section>
</template>

<script>
import { mapState } from 'vuex'

export default {
  name: 'Profile',

  components: {
  },
  computed: {
    ...mapState('suppliers', ['supplier']),
    ...mapState('supply-agreements', ['supplyAgreements']),
    ...mapState('consumers', ['consumer'])
  }
}
</script>
