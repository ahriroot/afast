<script lang="ts" setup>
import { NTable, NCard, NCheckbox, NSpace, NEllipsis, NFlex, NTooltip } from 'naive-ui'
import type { FieldDef } from '../types'

const props = defineProps<{
    schema: FieldDef
}>()
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
                                        <NCheckbox v-if="field.nullable" type="checkbox" :checked="true"
                                            label="Nullable" />
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
                    <td v-if="field.kind === 'number'">
                        number
                    </td>
                    <td v-else-if="field.kind === 'string'">
                        string
                    </td>
                    <td v-else-if="field.kind === 'boolean'">
                        boolean
                    </td>
                    <td v-else-if="field.kind === 'object'">
                        <JsonView :schema="{
                            name: field.name,
                            desc: field.desc,
                            tag: field.tag,
                            kind: 'object',
                            fields: field.fields
                        }"></JsonView>
                    </td>
                    <td v-else-if="field.kind === 'array'">
                        <template v-if="field.items.kind === 'number'">
                            number[]
                        </template>
                        <template v-else-if="field.items.kind === 'string'">
                            string[]
                        </template>
                        <template v-else-if="field.items.kind === 'boolean'">
                            boolean[]
                        </template>
                        <template v-else-if="field.items.kind === 'object'">
                            <JsonView :schema="{
                                name: field.name,
                                desc: field.desc,
                                tag: field.tag,
                                kind: 'object',
                                fields: field.items.fields
                            }">
                            </JsonView>
                        </template>
                    </td>
                    <td v-else-if="field.kind === 'enum'">
                        <NSpace vertical>
                            <template v-for="variant in field.variants" :key="variant.name">
                                <NCard v-if="variant.kind === 'unit'">
                                    {{ variant.name }}
                                </NCard>
                                <template v-else-if="variant.kind === 'object'">
                                    <JsonView :schema="{
                                        name: field.name,
                                        desc: field.desc,
                                        tag: field.tag,
                                        kind: 'object',
                                        fields: variant.fields
                                    }"></JsonView>
                                </template>
                            </template>
                        </NSpace>
                    </td>
                </tr>
            </tbody>
        </NTable>
    </template>
</template>

<style scoped>
.required {
    color: red;
}
</style>
