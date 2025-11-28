

export type Kind = { kind: 'unit' } | { kind: 'number' | 'i8' | 'i16' | 'i32' | 'i64' | 'i128' | 'u8' | 'u16' | 'u32' | 'u64' | 'u128' | 'f32' | 'f64' } | { kind: 'bool' } | { kind: 'string' } | { kind: 'object', fields: FieldDef[] } | { kind: 'array', items: Kind } | { kind: 'enum', variants: FieldDef[] }

export type FieldDef = {
    name: string
    desc: string
    nullable?: boolean
    tag: Tag | null
    alias?: string
} & Kind

export type Tag = {
    name?: string
    description: string
    required?: string
    min?: { value: number, unit: string }
    max?: { value: number, unit: string }
}
