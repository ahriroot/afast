import { Options } from '../../types'
import { Field } from './base'

export const NumberField = (options: Options = { primary: false, show: true }) => {
    options.type = 'Number'
    return Field(options)
}
