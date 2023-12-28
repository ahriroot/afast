import { Model } from '../model'
import { Field } from './base'

export class FieldForeign extends Field {
    constructor(options: { show?: boolean; nullable?: boolean; foreign: typeof Model; references?: string }) {
        super(options)
    }
}

export const fieldForeign = (options: {
    show?: boolean
    nullable?: boolean
    foreign: typeof Model
    references?: string
}) => {
    return new FieldForeign(options)
}

export class Station {
    model: typeof Model
    references: string
    constructor(options: { model: typeof Model; references?: string }) {
        this.model = options.model
        this.references = options.references || 'id'
    }
}

export const station = (options: { model: typeof Model; references?: string }) => {
    return new Station(options)
}
