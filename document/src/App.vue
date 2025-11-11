<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import { NConfigProvider, darkTheme, NCode, NButton, NGlobalStyle } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import json from 'highlight.js/lib/languages/json'
import JsonEditor from './components/JsonEditor.vue'
import { generateDefault } from './utils/default'

const baseURL = import.meta.env.VITE_BASE_URL

hljs.registerLanguage('json', json)

const services = ref<any[]>([])

const client = ref<any | null>(null)

async function loadModuleWithBlob(url: string) {
    try {
        const response = await fetch(url);
        const code = await response.text();

        const blob = new Blob([code], { type: 'application/javascript' });
        const blobUrl = URL.createObjectURL(blob);

        const module = await import(blobUrl);
        const AFastClient = module.AFastClient;

        URL.revokeObjectURL(blobUrl);
        return AFastClient;
    } catch (error) {
        console.error('加载失败:', error);
    }
}

onBeforeMount(async () => {
    const ss = await fetch(`${baseURL}/doc`)
    const svcs = await ss.json()
    for (let svc of svcs.services) {
        const AFastClient = await loadModuleWithBlob(`${baseURL}/code/${svc}/js`)
        client.value = new AFastClient({
            header: async () => {
                return {
                    id: 1,
                };
            },
            call: async (buf: Uint8Array): Promise<Uint8Array> => {
                const response = await fetch(`${baseURL}/api`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/octet-stream',
                    },
                    body: buf as any,
                });
                if (!response.ok) {
                    const body = await response.text();
                    throw new Error(`HTTP error: ${response.status} ${response.statusText} ${body}`);
                }
                const data = await response.arrayBuffer();
                return new Uint8Array(data);
            },
        });
        const schema = await fetch(`${baseURL}/doc/${svc}`)
        const schemaData = await schema.json()
        for (let resp of schemaData) {
            services.value.push({
                data: generateDefault(resp.request.fields as any),
                api: resp,
                resp: null,
                schema: resp.request,
                ns: resp.ns,
                name: resp.name
            })
        }
    }
})

function safeStringify(obj: any) {
    return JSON.stringify(obj, (_, value) => {
        if (typeof value === 'bigint') {
            return value.toString() + 'n'; // 或者直接 return value.toString()
        }
        return value;
    }, 4);
}


const submit = async (index: number) => {
    const { ns, name, data } = services.value[index]
    let fn: any = client.value
    for (let n of ns) {
        fn = fn[n]
    }
    if (fn && fn[name]) {
        try {
            const resp = await fn[name](data)
            services.value[index].resp = resp
        } catch (e: any) {
            services.value[index].resp = e.message || e.toString()
        }
    }
}
</script>

<template>
    <NConfigProvider :hljs="hljs" :theme="darkTheme">
        <div class="app">
            <div v-for="(i, index) in services">
                <h1>Name: {{ i.api.name || '-' }}</h1>
                <h2>Description: {{ i.api.desc || '-' }}</h2>
                <JsonEditor :schema="i.api.request" v-model:modelValue="i.data" />
                <NButton @click="submit(index)">Submit</NButton>
                <NCode :code="safeStringify(i.data)" :language="'json'" />
                <br />
                <NCode :code="safeStringify(i.resp)" :language="'json'" />
                <br />
            </div>
        </div>
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
    padding: 32px;
}
</style>
