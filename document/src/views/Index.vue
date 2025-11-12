<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import { useRouter } from 'vue-router'
import { NCard, NScrollbar, NSpace } from 'naive-ui'

const baseURL = import.meta.env.VITE_BASE_URL
const router = useRouter()
const services = ref<any>({})

onBeforeMount(async () => {
    const ss = await fetch(`${baseURL}/doc`)
    services.value = await ss.json()
})

const handleToService = (svc: any) => {
    router.push({ name: 'Service', params: { name: svc.name } })
}
</script>

<template>
    <div class="index">
        <NScrollbar>
            <div class="container">
                <div class="service" v-for="svc in services.services" @click="handleToService(svc)">
                    <NCard>
                        <NSpace align="center">
                            <h2>{{ svc.name }}</h2>
                            <p>{{ svc.desc }}</p>
                            <span>{{ svc.count }} Handlers</span>
                        </NSpace>
                    </NCard>
                </div>
            </div>
        </NScrollbar>
    </div>
</template>

<style scoped>
.index {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px 0;
}

.service {
    cursor: pointer;
}
</style>
