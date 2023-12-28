import { Field } from './base'

export class FieldPrimary extends Field {
    constructor({ show }: { show?: boolean }) {
        super({ primary: true, show })
    }
}

export const fieldPrimary = (options: { show?: boolean } = {}) => {
    return new FieldPrimary(options)
}
