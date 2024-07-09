<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated class="bg-accent">
      <q-toolbar>
        <q-btn flat dense round icon="menu" aria-label="Menu" @click="toggleLeftDrawer" />

        <q-toolbar-title>
          Inventory and Borrowing System
        </q-toolbar-title>

      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label
          header
        >
          BSHM
        </q-item-label>

        <EssentialLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import EssentialLink, { EssentialLinkProps } from 'components/EssentialLink.vue';

const essentialLinks: EssentialLinkProps[] = [
  {
    title: 'Home',
    icon: 'mdi-home',
    link_name: 'home',
    expand: false,
    expand_link: []
  },
  {
    title: 'Inventory',
    icon: 'mdi-list-box',
    expand: true,
    expand_link: [
      {
        title: 'Home',
        caption: 'Dashboard page',
        icon: 'mdi-home',
        link_name: 'tables'
      },
    ]
  },
  {
    title: 'Transactions',
    icon: 'chat',
    link_name: 'https://chat.quasar.dev',
    expand: false,
    expand_link: [
      {
        title: 'Borrow',
        icon: 'mdi-home',
        link_name: 'borrow'
      },
    ]

  },
  {
    title: 'Forum',
    caption: 'forum.quasar.dev',
    icon: 'record_voice_over',
    link_name: 'https://forum.quasar.dev',
    expand: false,
    expand_link: []
  },
  {
    title: 'Twitter',
    caption: '@quasarframework',
    icon: 'rss_feed',
    link_name: 'https://twitter.quasar.dev',
    expand: false,
    expand_link: []
  },
  {
    title: 'Facebook',
    caption: '@QuasarFramework',
    icon: 'public',
    link_name: 'https://facebook.quasar.dev',
    expand: false,
    expand_link: []
  },
  {
    title: 'Quasar Awesome',
    caption: 'Community Quasar projects',
    icon: 'favorite',
    link_name: 'https://awesome.quasar.dev',
    expand: false,
    expand_link: []
  }
];

const leftDrawerOpen = ref(false)

function toggleLeftDrawer() {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
</script>
