<template>
  <v-card outlined elevation="2">
      <v-card-text class="content">
        <v-row align="center">
        <v-col
          class="text-h5"
          cols="12"
        >
          {{ agreement.rate }} c/kWh
        </v-col>
      </v-row>
      </v-card-text>
      <v-divider></v-divider>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn text>
          Offer this supply agreement
        </v-btn>
      </v-card-actions>
    </v-card>
</template>

<script>
export default {
  name: 'SupplyAgreement',
  props: ['agreement']
}
</script>
