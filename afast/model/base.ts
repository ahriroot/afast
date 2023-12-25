import { Options } from '../types'

export class Field {
    length?: number
    show: boolean = true
    primary: boolean = false
    default?: any
    nullable?: boolean = false
    constructor(options: Options) {
        if (options.length !== undefined) {
            this.length = options.length
        }
        if (options.show !== undefined) {
            this.show = options.show
        }
        if (options.primary !== undefined) {
            this.primary = options.primary
        }
        this.default = options.default
        if (options.nullable !== undefined) {
            this.nullable = options.nullable
        }
    }
}
