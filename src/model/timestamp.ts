import { Field } from './base'

export class FieldTimestamp extends Field {
    constructor(options: { show?: boolean; default?: number | string | null; nullable?: boolean }) {
        super(options)
    }
}

export const fieldTimestamp = (
    options: { show?: boolean; default?: number | string | null; nullable?: boolean } = {}
) => {
    return new FieldTimestamp(options)
}
