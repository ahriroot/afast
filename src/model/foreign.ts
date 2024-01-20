import { Options } from '../../types'
import { Field } from './base'

export const ForeignField = (options: Options = { primary: false, show: true }) => {
    options.type = 'Foreign'
    return Field(options)
}
