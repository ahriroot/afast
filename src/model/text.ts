import { Field } from './base'

export class FieldText extends Field {
    constructor(options: { show?: boolean; default?: string | null; nullable?: boolean }) {
        super(options)
    }
}

export const fieldText = (options: { show?: boolean; default?: string | null; nullable?: boolean } = {}) => {
    return new FieldText(options)
}
