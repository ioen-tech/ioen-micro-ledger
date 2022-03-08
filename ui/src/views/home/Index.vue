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
              align="center"
              class="mx-auto"
              justify="center"
            >
              <v-col
                class="text-center"
                cols="12"
                tag="h1"
              >
                <span
                  :class="[$vuetify.breakpoint.smAndDown ? 'display-3': 'display-4']"
                  class="font-weight-black"
                >
                  Join The Internet Of Energy Network
                </span>
              </v-col>
              <v-btn
                class="align-self-end ma-5"
                color="purple"
                outlined
                x-large
                to="/supplier">
                <v-icon size="40" class="mr-2">mdi-solar-panel-large</v-icon>
                suppliers
              </v-btn>
              <v-btn
                class="align-self-end ma-5"
                outlined
                color="green"
                x-large
                to="/consumer">
                <v-icon size="40" class="mr-2">mdi-account-outline</v-icon>
                Consumers
              </v-btn>
            </v-row>
          </v-container>
        </v-theme-provider>
      </v-img>
    </v-row>
  </section>
</template>

<script>
export default {
  name: 'Home',

  components: {
  }
}
</script>
