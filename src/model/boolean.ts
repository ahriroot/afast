import { Field } from './base'

export class FieldBoolean extends Field {
    constructor(options: { show?: boolean; default?: boolean | null; nullable?: boolean }) {
        super(options)
    }
}

export const fieldBoolean = (options: { show?: boolean; default?: boolean | null; nullable?: boolean } = {}) => {
    return new FieldBoolean(options)
}
