<template>
  <q-expansion-item v-if="expand_link && expand" :content-inset-level="0.5" expand-separator :icon="icon" :label="title">
    <q-expansion-item expand-separator :content-inset-level="0.5" icon="receipt" label="Receipts">

      <q-expansion-item v-for="(item, index) in expand_link" :key="index" label="item.title" :content-inset-level="0.5">
        <q-card>
          <q-card-section>
            Lorem ipsum dolor sit amet, consectetur adipisicing elit. Quidem, eius reprehenderit eos corrupti
            commodi magni quaerat ex numquam, dolorum officiis modi facere maiores architecto suscipit iste
            eveniet doloribus ullam aliquid.
          </q-card-section>
        </q-card>
      </q-expansion-item>

    </q-expansion-item>

  </q-expansion-item>

  <q-item
    v-else
    clickable
    :to="{ name: link_name }"
  >
    <q-item-section
      v-if="icon"
      avatar
    >
      <q-icon :name="icon" />
    </q-item-section>

    <q-item-section>
      <q-item-label>{{ title }}</q-item-label>
    </q-item-section>
  </q-item>
</template>

<script setup lang="ts">
  import { EssentialLinkProps } from './models';

  withDefaults(defineProps<EssentialLinkProps>(), {
    caption: '',
    link_name: '',
    icon: '',
    expand: false,
    expand_link: [],
  });
</script>
