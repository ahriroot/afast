

export type Kind = { kind: 'unit' } | { kind: 'number' } | { kind: 'boolean' } | { kind: 'string' } | { kind: 'object', fields: FieldDef[] } | { kind: 'array', items: Kind } | { kind: 'enum', variants: FieldDef[] }

export type FieldDef = {
    name: string
    desc: string
    nullable?: boolean
    tag: Tag|null
} & Kind

export type Tag = {
    name?: string
    description: string
    required?: string
    min?: { value: number, unit: string }
    max?: { value: number, unit: string }
}
