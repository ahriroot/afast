import { Field } from './base'

export class FieldString extends Field {
    constructor(options: { show?: boolean; length?: number; default?: string | null; nullable?: boolean }) {
        super(options)
    }
}

export const fieldString = (
    options: { show?: boolean; length?: number; default?: string | null; nullable?: boolean } = {
        length: 255,
    }
) => {
    return new FieldString(options)
}
