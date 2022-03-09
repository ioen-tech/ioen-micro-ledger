<template>
  <v-app>
    <v-app-bar
      app
      color="white"
    >
      <a href="/" style="text-decoration: none; color: black;">
        <div class="d-flex align-center">
          <v-img
            alt="Vuetify Logo"
            class="shrink mr-2"
            contain
            :src="require('@/assets/logo.svg')"
            transition="scale-transition"
            width="40"
          />
          <h2>Internet Of Energy</h2>
        </div>
      </a>
      <v-spacer></v-spacer>
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
