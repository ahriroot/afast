import { Field } from './base'

export class FieldJson extends Field {
    constructor(options: { show?: boolean; default?: any; nullable?: boolean }) {
        super(options)
    }
}

export const fieldJson = (options: { show?: boolean; default?: any; nullable?: boolean } = {}) => {
    return new FieldJson(options)
}
