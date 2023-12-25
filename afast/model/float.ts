import { Field } from './base'

export class FieldFloat extends Field {
    constructor(options: { show?: boolean; default?: number | null; nullable?: boolean }) {
        super(options)
    }
}

export const fieldFloat = (options: { show?: boolean; default?: number | null; nullable?: boolean } = {}) => {
    return new FieldFloat(options)
}
