import { Options } from '../../types'
import { Field } from './base'

export const TextField = (options: Options = { primary: false, show: true }) => {
    options.type = 'Text'
    return Field(options)
}
