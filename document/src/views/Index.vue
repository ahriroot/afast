<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import { useRouter } from 'vue-router'
import { NButton, NCard, NModal, NScrollbar, NSpace, NTabs, NTabPane, NCode } from 'naive-ui'

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

const showExample = ref(false)
const jsCode = `import { AFastClient } from 'xxx';

const client = new AFastClient({
    header: async () => {
        return {
            token: () => { return localStorage.getItem('token') },
        }
    },
    hook: async (header) => {
        console.log('hook:', header);
    },
    call: async (buf) => {
        console.log(buf);
        const response = await fetch('http://host/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/octet-stream',
            },
            body: buf,
        });
        if (!response.ok) {
            const body = await response.text();
            throw new Error(\`HTTP error: \${response.status} \${response.statusText} \${body}\`);
        }
        const data = await response.arrayBuffer();
        return new Uint8Array(data);
    },
});

const response = await client[.namespace1.namespace2].handle({});`
const tsCode = `import { AFastClient } from 'xxx';

const client = new AFastClient({
    header: async () => {
        return {
            token: () => { return localStorage.getItem('token') },
        }
    },
    hook: async (header: {token: string}) => {
        console.log('hook:', header);
    },
    call: async (buf: Uint8Array) => {
        console.log(buf);
        const response = await fetch('http://host/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/octet-stream',
            },
            body: buf as any,
        });
        if (!response.ok) {
            const body = await response.text();
            throw new Error(\`HTTP error: \${response.status} \${response.statusText} \${body}\`);
        }
        const data = await response.arrayBuffer();
        return new Uint8Array(data);
    },
});

const response = await client[.namespace1.namespace2].handle({});`
</script>

<template>
    <div class="index">
        <NScrollbar>
            <div class="container">
                <NModal :show="showExample" style="width: 900px;" @close="showExample = false">
                    <NCard>
                        <h1>1: Get client code</h1>
                        <p>http://host/code/{service}/{lang}</p>
                        <p>example: http://host/code/service1/js</p>
                        <h1>2: Create client</h1>
                        <NTabs type="segment" animated>
                            <NTabPane name="js" tab="js">
                                <NCode :code="jsCode" :language="'javascript'" />
                            </NTabPane>
                            <NTabPane name="ts" tab="ts">
                                <NCode :code="tsCode" :language="'javascript'" />
                            </NTabPane>
                        </NTabs>
                    </NCard>
                </NModal>
                <NSpace justify="end">
                    <NButton @click="showExample = true" secondary>Example</NButton>
                </NSpace>
                <br />
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
