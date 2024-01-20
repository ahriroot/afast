import { Options } from '../../types'
import { Field } from './base'

export const JsonField = (options: Options = { show: true }) => {
    options.primary = true
    options.type = 'Json'
    return Field(options)
}
