

export type Kind = { kind: 'unit' } | { kind: 'number' } | { kind: 'boolean' } | { kind: 'string' } | { kind: 'object', fields: FieldDef[] } | { kind: 'array', items: Kind } | { kind: 'enum', variants: FieldDef[] }

export type FieldDef = {
    name: string
    nullable?: boolean
} & Kind
