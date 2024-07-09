<template>
  <div class="column q-gutter-y-md items-center justify-center q-pa-md window-height">
    <div class="column justify-center q-gutter-y-md shadow-12 q-pa-md" style="width: 45%; border-radius: 15px;">
      <div class="row items-center">
        <img src="https://picsum.photos/1200/200" class="col-3" style="border-radius: 50%; height: 100px;" />
        <span class="col text-weight-bold text-h5 text-center">BSHM: <small>inventory and borrowing</small></span>
      </div>
      <q-form class="column q-gutter-y-md">
        <span class="text-weight-normal text-h5">Signin to your account</span>
        <q-input outlined v-model="text" color="accent" label="Username" class="col" :rules="[ val => !!val || 'Please enter username' ]" />
        <q-input outlined v-model="text" color="accent" label="Password" class="col" :rules="[ val => !!val || 'Please enter password' ]" />
        <q-btn type="submit" label="signin" color="accent" class="col q-py-xs" />
      </q-form>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from 'vue';

  const text = ref('');
</script>
