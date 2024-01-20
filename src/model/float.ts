import { Options } from '../../types'
import { Field } from './base'

export const FloatField = (options: Options = { primary: false, show: true }) => {
    options.type = 'Float'
    return Field(options)
}
