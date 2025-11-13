<script setup lang="ts">
import { NConfigProvider, NMessageProvider, darkTheme, NGlobalStyle } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import json from 'highlight.js/lib/languages/json'

hljs.registerLanguage('json', json)
</script>

<template>
    <NConfigProvider :hljs="hljs" :theme="darkTheme">
        <NMessageProvider keep-alive-on-hover>
            <div class="app">
                <router-view />
            </div>
        </NMessageProvider>
        <NGlobalStyle />
    </NConfigProvider>
</template>

<style scoped>
.app {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}
</style>
