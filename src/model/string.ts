import { Options } from '../../types'
import { Field } from './base'

export const StringField = (options: Options = { primary: false, show: true, length: 255 }) => {
    options.type = 'String'
    return Field(options)
}
