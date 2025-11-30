<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NCode, NInput, NButton, NCollapse, NCollapseItem, NScrollbar, NSpace, useMessage, useNotification, NCheckbox, NCard, NInputGroup, NModal, NTabs, NTabPane } from 'naive-ui'
import { Menu } from '@vicons/ionicons5'
import hljs from 'highlight.js/lib/core'
import json from 'highlight.js/lib/languages/json'
import JsonEditor from '../components/JsonEditor.vue'
import JsonView from '../components/JsonView.vue'
import { generateDefault } from '../utils/default'

hljs.registerLanguage('json', json)
const baseURL = import.meta.env.VITE_BASE_URL
const message = useMessage()
const notification = useNotification()
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

const editAuthorization = ref(false)
const authorization = ref(localStorage.getItem('authorization') || '')
const handleEditAuthorization = (e: MouseEvent) => {
    e.stopPropagation()
    editAuthorization.value = true
}
const handleSaveAuthorization = (e: MouseEvent) => {
    e.stopPropagation()
    localStorage.setItem('authorization', authorization.value)
    editAuthorization.value = false
}

const expandedNames = ref<number[]>([])
const handleExpandedNames = (names: number[]) => {
    localStorage.setItem('expandedNames', JSON.stringify({
        name: route.params.name,
        names: names,
    }))
    expandedNames.value = names
}

onBeforeMount(async () => {
    const names = localStorage.getItem('expandedNames')
    if (names) {
        try {
            const res = JSON.parse(names)
            if (res.name === route.params.name) {
                expandedNames.value = res.names
            }
        } catch (_) { }
    }
    try {
        const svc = route.params.name
        const [AFastClient, AFastValidateError] = await loadModuleWithBlob(`${baseURL}/code/${svc}/js`)
        client.value = new AFastClient({
            header: async () => {
                return {
                    token: authorization.value,
                };
            },
            hook: async (header: any) => {
                console.log('hook:', header);
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
                data: generateDefault(resp.request),
                api: resp,
                resp: null,
                preview: false,
            })
        }
    } catch (e: any) {
        message.error(e.message || e.toString())
        router.push('/')
    }
})

function safeStringify(obj: any) {
    return JSON.stringify(obj, null, 4);
}

const processings = ref<any>({})

const submit = async (index: number) => {
    const { api, data } = services.value[index]
    let fn: any = client.value
    for (let n of api.ns) {
        fn = fn[n]
    }
    if (fn && fn[api.name]) {
        try {
            processings.value[index] = true
            const resp = await fn[api.name](data)
            services.value[index].resp = resp
        } catch (e: any) {
            if (error.value && e instanceof error.value) {
                message.warning(e.message || e.toString())
            } else {
                // message.error(e.message || e.toString())
                notification.error({
                    title: 'Error',
                    content: e.message || e.toString(),
                    duration: 5000,
                    keepAliveOnHover: true,
                })
            }
        } finally {
            processings.value[index] = false
        }
    } else {
        message.error(`API ${api.name} not found`)
    }
}

const filter = ref('')

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

const openSwaggerUI = () => {
    const serviceName = route.params.name as string
    const swaggerUrl = `${baseURL}/swagger/${serviceName}`
    // Open Swagger UI using swagger-ui web service
    window.open(`https://petstore.swagger.io/?url=${encodeURIComponent(swaggerUrl)}`, '_blank')
}
</script>

<template>
    <div class="service">
        <NScrollbar>
            <div class="container">
                <NModal :show="showExample" style="width: 900px;" preset="dialog" @close="showExample = false">
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
                </NModal>
                <NSpace align="center" justify="space-between">
                    <NSpace align="center" justify="end">
                        <NButton @click="handleExpandedNames([])" secondary>
                            <template #icon>
                                <Menu />
                            </template>
                        </NButton>
                    </NSpace>
                    <NSpace align="center" justify="end">
                        <NInput v-model:value="filter" placeholder="Filter name / desc / namespace"
                            style="width: 260px;" />
                        <NButton @click="showExample = true" secondary>Example</NButton>
                        <NButton @click="openSwaggerUI" secondary>Swagger UI</NButton>
                        <NButton v-show="!editAuthorization" @click="handleEditAuthorization" secondary>
                            Authorization
                        </NButton>
                        <NInputGroup v-show="editAuthorization">
                            <NInput v-model:value="authorization" @click="(e) => e.stopPropagation()"
                                style="width: 460px;" />
                            <NButton @click="handleSaveAuthorization" secondary>
                                Save
                            </NButton>
                        </NInputGroup>
                    </NSpace>
                </NSpace>
                <NCollapse :expanded-names="expandedNames" :on-update:expanded-names="handleExpandedNames">
                    <NCollapseItem v-for="(i, index) in services" :title="i.api.desc || '-'" :name="index"
                        v-show="filter === '' || i.api.name.includes(filter) || i.api.desc.includes(filter) || i.api.ns.includes(filter)">
                        <template #header>
                            <NSpace align="center">
                                <h2>{{ i.api.name || '' }}</h2>
                                <span>{{ i.api.desc || '' }}</span>
                            </NSpace>
                        </template>
                        <template #header-extra>
                            <span>Namespace: {{ i.api.ns || '[]' }}</span>
                        </template>
                        <div class="api">
                            <NSpace vertical>
                                <JsonEditor :schema="i.api.request" v-model:modelValue="i.data" />
                                <NSpace align="center" justify="end">
                                    <NCheckbox v-model:checked="i.preview" label="Preview Request" />
                                    <NButton @click="submit(index)" :loading="processings[index]">Send</NButton>
                                </NSpace>
                                <NCard title="Request" v-if="i.preview">
                                    <NCode :code="safeStringify(i.data)" :language="'json'" word-wrap />
                                    <NCard>
                                        {{ i.api.req_type }}
                                    </NCard>
                                </NCard>
                                <NCard title="Response">
                                    <NCode :code="safeStringify(i.resp)" :language="'json'" word-wrap />
                                    <NCard>
                                        {{ i.api.resp_type }}
                                    </NCard>
                                    <JsonView :schema="i.api.response" />
                                </NCard>
                            </NSpace>
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
    max-width: 1000px;
    margin: 0 auto;
    padding: 20px 0;
}

.api {
    padding: 0 32px;
}
</style>
