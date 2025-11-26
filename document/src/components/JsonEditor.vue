<script lang="ts" setup>
import { ref, watch } from 'vue'
import { NTable, NInputGroup, NInput, NInputNumber, NCheckbox, NRadioGroup, NRadio, NSpace, NButton, NEllipsis, NFlex, NTooltip } from 'naive-ui'
import { generateDefaults } from '../utils/default'
import type { FieldDef, Kind } from '../types'

const props = defineProps<{
    schema: FieldDef
    modelValue: Record<string, any>
}>()

const emit = defineEmits<{
    (e: 'update:modelValue', value: Record<string, any>): void
}>()

const data = ref<Record<string, any>>(props.modelValue)
watch(() => props.modelValue, val => {
    data.value = val
})

const addArrayItem = (field: FieldDef, fields: FieldDef[]) => {
    data.value[field.name].push(generateDefaults(fields))
}

const changeNull = (checked: boolean, field: FieldDef) => {
    data.value[field.name] = checked ? null : generateDefaults([field])[field.name]
}

const changeEnumVariant = (field: FieldDef | null, variant: Kind, index: number) => {
    if (field) {
        if (variant.kind === 'object') {
            data.value[field.name] = { _type: index, ...generateDefaults(variant.fields) }
        } else if (variant.kind === 'unit') {
            data.value[field.name] = { _type: index }
        } else {
            throw new Error('Unsupported variant kind')
        }
    } else {
        if (variant.kind === 'object') {
            data.value = { _type: index, ...generateDefaults(variant.fields) }
            emit('update:modelValue', data.value)
        } else if (variant.kind === 'unit') {
            data.value = { _type: index }
            emit('update:modelValue', data.value)
        } else {
            throw new Error('Unsupported variant kind')
        }
    }
}
</script>

<template>
    <template v-if="props.schema.kind === 'object'">
        <NTable :bordered="true" :single-line="false" size="small">
            <tbody>
                <tr v-for="field in props.schema.fields" :key="field.name">
                    <td style="vertical-align: top; width: 200px;">
                        <NTooltip>
                            <template #trigger>
                                <NFlex vertical :gap="0">
                                    <NSpace align="center">
                                        <NSpace align="center">
                                            <h3>{{ field.name }}</h3>
                                            <span class="required" v-if="field?.tag?.required">*</span>
                                        </NSpace>
                                        <NCheckbox v-if="field.nullable" type="checkbox"
                                            @update:checked="e => changeNull(e, field)" label="Null" />
                                    </NSpace>
                                    <NEllipsis style="width: 200px; font-size: 12px;" :tooltip="false">
                                        {{ field?.tag?.description || '-' }}
                                    </NEllipsis>
                                </NFlex>
                            </template>
                            <div v-if="field?.tag?.description">{{ field?.tag?.description }}</div>
                            <div v-if="field?.tag?.min">Min: {{ field.tag.min.value }}</div>
                            <div v-if="field?.tag?.max">Max: {{ field.tag.max.value }}</div>
                        </NTooltip>
                    </td>
                    <td v-if="data[field.name] === null"></td>
                    <td v-else-if="field.kind === 'number'">
                        <NInputNumber :value="data[field.name]" @update:value="val => data[field.name] = val ?? 0"
                            :min="field?.tag?.min?.value || undefined" :max="field?.tag?.max?.value || undefined"
                            :placeholder="field.name" />
                    </td>
                    <td v-else-if="field.kind === 'string'">
                        <NInput v-model:value="data[field.name]" :placeholder="field.name" />
                    </td>
                    <td v-else-if="field.kind === 'boolean'">
                        <NCheckbox type="checkbox" v-model:checked="data[field.name]" />
                    </td>
                    <td v-else-if="field.kind === 'object'">
                        <JsonEditor :schema="{
                            name: field.name,
                            desc: field.desc,
                            tag: field.tag,
                            kind: 'object',
                            fields: field.fields
                        }" v-model:modelValue="data[field.name]"></JsonEditor>
                    </td>
                    <td v-else-if="field.kind === 'array'">
                        <template v-if="field.items.kind === 'number'">
                            <div v-for="(_, index) in data[field.name]">
                                <NInputGroup>
                                    <NInputNumber v-model:value="data[field.name][index]"
                                        :min="field?.tag?.min?.value || undefined"
                                        :max="field?.tag?.max?.value || undefined"
                                        :placeholder="`${field.name} - ${index}`" />
                                    <NButton @click="data[field.name].splice(index, 1)">-</NButton>
                                </NInputGroup>
                            </div>
                            <NButton @click="data[field.name].push(0)">+</NButton>
                        </template>
                        <template v-else-if="field.items.kind === 'string'">
                            <div v-for="(_, index) in data[field.name]">
                                <NInputGroup>
                                    <NInput v-model:value="data[field.name][index]"
                                        :placeholder="`${field.name} - ${index}`" />
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
                                    desc: field.desc,
                                    tag: field.tag,
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
                        <template v-if="field.variants[data[field.name]._type]?.kind === 'object'">
                            <br />
                            <br />
                            <JsonEditor :schema="{
                                name: field.name,
                                desc: field.desc,
                                tag: field.tag,
                                kind: 'object',
                                fields: (field.variants[data[field.name]._type]! as any).fields
                            }" v-model:modelValue="data[field.name]"></JsonEditor>
                        </template>
                    </td>
                </tr>
            </tbody>
        </NTable>
    </template>
    <template v-else-if="props.schema.kind === 'enum'">
        <NRadioGroup v-model:value="data['_type']" name="radiogroup">
            <NSpace>
                <NRadio v-for="(variant, index) in props.schema.variants" :key="index" :value="index"
                    @change="changeEnumVariant(null, variant, index)">
                    {{ index }}
                </NRadio>
            </NSpace>
        </NRadioGroup>
        <template v-if="props.schema.variants[data['_type']]?.kind === 'object'">
            <br />
            <br />
            <JsonEditor :schema="{
                name: props.schema.name,
                desc: props.schema.desc,
                tag: props.schema.tag,
                kind: 'object',
                fields: (props.schema.variants[data['_type']]! as any).fields
            }" v-model:modelValue="data"></JsonEditor>
        </template>
    </template>
</template>

<style scoped>
.required {
    color: red;
}
</style>
