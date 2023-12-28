import { Field } from './base'

export class FieldNumber extends Field {
    constructor(options: { show?: boolean; default?: number | null; nullable?: boolean }) {
        super(options)
    }
}

export const fieldNumber = (options: { show?: boolean; default?: number | null; nullable?: boolean } = {}) => {
    return new FieldNumber(options)
}
