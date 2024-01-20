import { Options } from '../../types'
import { Field } from './base'

export const DateField = (options: Options = { primary: false, show: true }) => {
    options.type = 'Date'
    return Field(options)
}
