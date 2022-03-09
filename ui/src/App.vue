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
      <v-icon v-if="supplier.entryHash !== undefined">
        mdi-account
      </v-icon>
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
    ...mapState('suppliers', ['supplier'])
  }
}
</script>
