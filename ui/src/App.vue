<template>
  <v-app>
    <v-app-bar
      app
      color="white"
    >
      <v-btn x-large icon to="/">
        <v-img
          alt="IOEN Logo"
          class="shrink"
          :src="require('@/assets/logo.svg')"
          width="40"
        />
      </v-btn><h2>Internet Of Energy</h2>
      <v-spacer></v-spacer>
      <v-btn
        to="/demo-setup"
        icon
      >
        <v-icon>
          mdi-arrow-right-circle
        </v-icon>
      </v-btn>
      <v-btn
        v-if="showProfile()"
        x-large
        icon
        to="/profile"
      >
        <v-icon>
          mdi-account
        </v-icon>
      </v-btn>
      <v-icon v-else>
          mdi-login
        </v-icon>
    </v-app-bar>
    <v-main>
      <router-view/>
    </v-main>
  </v-app>
</template>

<script>
import { mapState } from 'vuex'

export default {
  name: 'App',
  computed: {
    ...mapState('suppliers', ['supplier']),
    ...mapState('consumers', ['consumer'])
  },
  methods: {
    showProfile () {
      if (this.supplier.address !== undefined || this.consumer.address !== undefined) {
        return true
      }
    }
  },
  created () {
    this.$store.dispatch('suppliers/initialise')
    this.$store.dispatch('consumers/initialise')
  }
}
</script>
