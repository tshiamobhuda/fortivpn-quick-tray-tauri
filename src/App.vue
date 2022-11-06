<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
    import ToolbarActions from './components/toolbar-actions.vue';
</script>

<template>
    <q-layout view="hHh lpR fFf">
        <q-toolbar class="bg-primary text-white q-px-md">
            <q-toolbar-title>FortiVPN Quick Tray</q-toolbar-title>

            <toolbar-actions />
        </q-toolbar>

        <q-page-container>
            <q-page padding />
        </q-page-container>
    </q-layout>
</template>
