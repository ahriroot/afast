<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NCode, NButton, NCollapse, NCollapseItem, NScrollbar, NSpace, useMessage } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import json from 'highlight.js/lib/languages/json'
import JsonEditor from '../components/JsonEditor.vue'
import { generateDefault } from '../utils/default'

hljs.registerLanguage('json', json)
const baseURL = import.meta.env.VITE_BASE_URL
const message = useMessage()
const route = useRoute()
const router = useRouter()
const services = ref<any[]>([])
const client = ref<any | null>(null)
const error = ref<any | null>(null)

async function loadModuleWithBlob(url: string): Promise<[any, any]> {
    try {
        const response = await fetch(url);
        const code = await response.text();

        const blob = new Blob([code], { type: 'application/javascript' });
        const blobUrl = URL.createObjectURL(blob);

        const module = await import(blobUrl);
        const AFastClient = module.AFastClient;
        const AFastValidateError = module.AFastValidateError;

        URL.revokeObjectURL(blobUrl);
        return [AFastClient, AFastValidateError];
    } catch (error) {
        console.error('加载失败:', error);
        throw error;
    }
}

onBeforeMount(async () => {
    try {
        const svc = route.params.name
        const [AFastClient, AFastValidateError] = await loadModuleWithBlob(`${baseURL}/code/${svc}/js`)
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
        error.value = AFastValidateError
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
    } catch (e: any) {
        message.error(e.message || e.toString())
        router.push('/')
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
            if (error.value && e instanceof error.value) {
                message.warning(e.message || e.toString())
            } else {
                message.error(e.message || e.toString())
            }
        }
    }
}
</script>

<template>
    <div class="service">
        <NScrollbar>
            <div class="container">
                <NCollapse>
                    <NCollapseItem v-for="(i, index) in services" :title="i.api.desc || '-'" :name="index">
                        <template #header>
                            <NSpace align="center">
                                <h2>{{ i.api.name || '' }}</h2>
                                <span>{{ i.api.desc || '' }}</span>
                            </NSpace>
                        </template>
                        <template #header-extra>
                            123
                        </template>
                        <div class="api">
                            <JsonEditor :schema="i.api.request" v-model:modelValue="i.data" />
                            <NButton @click="submit(index)">Submit</NButton>
                            <NCode :code="safeStringify(i.data)" :language="'json'" />
                            <br />
                            <NCode :code="safeStringify(i.resp)" :language="'json'" />
                            <br />
                        </div>
                    </NCollapseItem>
                </NCollapse>
            </div>
        </NScrollbar>
    </div>
</template>

<style scoped>
.service {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px 0;
}

.api {
    padding: 0 32px;
}
</style>
