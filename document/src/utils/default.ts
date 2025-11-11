import type { FieldDef } from '../types'

export const generateDefault = (schema: FieldDef[]): Record<string, any> => {
    const result: Record<string, any> = {}
    for (const field of schema) {
        switch (field.kind) {
            case 'object':
                result[field.name] = generateDefault(field.fields)
                break
            case 'array':
                result[field.name] = []
                break
            case 'number':
                result[field.name] = 0
                break
            case 'boolean':
                result[field.name] = false
                break
            case 'string':
                result[field.name] = ''
                break
            case 'enum':
                switch (field.variants[0]!.kind) {
                    case 'object':
                        result[field.name] = {_type:0,...generateDefault(field.variants[0]!.fields)}
                        break
                    case 'array':
                        result[field.name] = []
                        break
                    case 'number':
                        result[field.name] = 0
                        break
                    case 'boolean':
                        result[field.name] = false
                        break
                    case 'string':
                        result[field.name] = ''
                        break
                    default:
                        break
                }
                break
            default:
                break
        }
    }
    return result
}