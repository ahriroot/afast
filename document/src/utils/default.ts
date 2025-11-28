import type { FieldDef } from '../types'

export const generateDefault = (field: FieldDef): any => {
    switch (field.kind) {
        case 'object':
            const objResult: Record<string, any> = {}
            for (const subField of field.fields) {
                objResult[subField.name] = generateDefault(subField)
            }
            return objResult
        case 'array':
            return []
        case 'number':
            return field.tag?.min?.value || 0
        case 'i8':
            return field.tag?.min?.value || 0
        case 'i16':
            return field.tag?.min?.value || 0
        case 'i32':
            return field.tag?.min?.value || 0
        case 'i64':
            return field.tag?.min?.value || 0
        case 'i128':
            return field.tag?.min?.value || 0
        case 'u8':
            return field.tag?.min?.value || 0
        case 'u16':
            return field.tag?.min?.value || 0
        case 'u32':
            return field.tag?.min?.value || 0
        case 'u64':
            return field.tag?.min?.value || 0
        case 'u128':
            return field.tag?.min?.value || 0
        case 'f32':
            return field.tag?.min?.value || 0
        case 'f64':
            return field.tag?.min?.value || 0
        case 'bool':
            return false
        case 'string':
            return ''
        case 'enum':
            if (field.variants.length > 0) {
                const firstVariant = field.variants[0]!
                switch (firstVariant.kind) {
                    case 'object':
                        return { _type: 0, ...generateDefault(firstVariant) }
                    case 'unit':
                        return { _type: 0 }
                    default:
                        return { _type: 0 }
                }
            }
            return { _type: 0 }
        case 'unit':
            return null
        default:
            return null
    }
}

export const generateDefaults = (schema: FieldDef[]): Record<string, any> => {
    const result: Record<string, any> = {}
    for (const field of schema) {
        result[field.name] = generateDefault(field)
    }
    return result
}
