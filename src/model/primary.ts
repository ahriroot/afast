import { Options } from '../../types'
import { Field } from './base'

export const PrimaryField = (options: Options = { show: true }) => {
    options.primary = true
    options.type = 'Primary'
    return Field(options)
}
