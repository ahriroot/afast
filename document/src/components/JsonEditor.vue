<script lang="ts" setup>
import { nextTick, ref } from 'vue'
import { NTable, NInputGroup, NInput, NInputNumber, NCheckbox, NRadioGroup, NRadio, NSpace, NButton } from 'naive-ui'
import { generateDefault } from '../utils/default'
import type { FieldDef, Kind } from '../types'

const props = defineProps<{
    schema: FieldDef
    modelValue: Record<string, any>
}>()

const data = ref<Record<string, any>>(props.modelValue)

const addArrayItem = (field: FieldDef, fields: FieldDef[]) => {
    data.value[field.name].push(generateDefault(fields))
}

const changeNull = (checked: boolean, field: FieldDef) => {
    data.value[field.name] = checked ? null : generateDefault([field])[field.name]
}

const showEnum = ref(true)

const changeEnumVariant = (field: FieldDef, variant: Kind, index: number) => {
    showEnum.value = false
    if (variant.kind === 'object') {
        data.value[field.name] = { _type: index, ...generateDefault(variant.fields) }
    } else if (variant.kind === 'array') {
        data.value[field.name] = []
    } else if (variant.kind === 'number') {
        data.value[field.name] = 0
    } else if (variant.kind === 'boolean') {
        data.value[field.name] = false
    } else if (variant.kind === 'string') {
        data.value[field.name] = ''
    } else {
        throw new Error('Unsupported variant kind')
    }
    nextTick(() => {
        showEnum.value = true
    })
}
</script>

<template>
    <template v-if="props.schema.kind === 'object'">
        <NTable :bordered="true" :single-line="false" size="small">
            <tbody>
                <tr v-for="field in props.schema.fields" :key="field.name">
                    <td style="vertical-align: top; width: 200px;">
                        <span>{{ field.name }}</span>
                        <NCheckbox v-if="field.nullable" type="checkbox" @update:checked="e => changeNull(e, field)" />
                    </td>
                    <td v-if="data[field.name] === null"></td>
                    <td v-else-if="field.kind === 'number'">
                        <NInputNumber v-model:value="data[field.name]" />
                    </td>
                    <td v-else-if="field.kind === 'string'">
                        <NInput v-model:value="data[field.name]" />
                    </td>
                    <td v-else-if="field.kind === 'boolean'">
                        <NCheckbox type="checkbox" v-model:checked="data[field.name]" />
                    </td>
                    <td v-else-if="field.kind === 'object'">
                        <JsonEditor :schema="{
                            name: field.name,
                            kind: 'object',
                            fields: field.fields
                        }" v-model:modelValue="data[field.name]"></JsonEditor>
                    </td>
                    <td v-else-if="field.kind === 'array'">
                        <template v-if="field.items.kind === 'number'">
                            <div v-for="(_, index) in data[field.name]">
                                <NInputGroup>
                                    <NInputNumber v-model:value="data[field.name][index]" />
                                    <NButton @click="data[field.name].splice(index, 1)">-</NButton>
                                </NInputGroup>
                            </div>
                            <NButton @click="data[field.name].push(0)">+</NButton>
                        </template>
                        <template v-else-if="field.items.kind === 'string'">
                            <div v-for="(_, index) in data[field.name]">
                                <NInputGroup>
                                    <NInput v-model:value="data[field.name][index]" />
                                    <NButton @click="data[field.name].splice(index, 1)">-</NButton>
                                </NInputGroup>
                            </div>
                            <NButton @click="data[field.name].push('')">+</NButton>
                        </template>
                        <template v-else-if="field.items.kind === 'boolean'">
                            <div v-for="(_, index) in data[field.name]">
                                <NInputGroup>
                                    <NCheckbox type="checkbox" v-model:checked="data[field.name][index]" />
                                    <NButton @click="data[field.name].splice(index, 1)">-</NButton>
                                </NInputGroup>
                            </div>
                            <NButton @click="data[field.name].push(false)">+</NButton>
                        </template>
                        <template v-else-if="field.items.kind === 'object'">
                            <div v-for="(_, index) in data[field.name]">
                                <JsonEditor :schema="{
                                    name: field.name,
                                    kind: 'object',
                                    fields: field.items.fields
                                }" v-model:modelValue="data[field.name][index]">
                                </JsonEditor>
                                <NButton @click="data[field.name].splice(index, 1)">-</NButton>
                            </div>
                            <NButton @click="addArrayItem(field, field.items.fields)">+</NButton>
                        </template>
                    </td>
                    <td v-else-if="field.kind === 'enum'">
                        <NRadioGroup v-model:value="data[field.name]['_type']" name="radiogroup">
                            <NSpace>
                                <NRadio v-for="(variant, index) in field.variants" :key="index" :value="index"
                                    @change="changeEnumVariant(field, variant, index)">
                                    {{ index }}
                                </NRadio>
                            </NSpace>
                        </NRadioGroup>
                        <template v-if="showEnum && field.variants[data[field.name]._type]?.kind === 'object'">
                            <JsonEditor :schema="{
                                name: field.name,
                                kind: 'object',
                                fields: (field.variants[data[field.name]._type]! as any).fields
                            }" v-model:modelValue="data[field.name]"></JsonEditor>
                        </template>
                    </td>
                </tr>
            </tbody>
        </NTable>
    </template>
</template>

<style scoped></style>
