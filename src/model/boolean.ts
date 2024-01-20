import { Options } from '../../types'
import { Field } from './base'

export const BooleanField = (options: Options = { primary: false, show: true }) => {
    options.type = 'Boolean'
    return Field(options)
}
