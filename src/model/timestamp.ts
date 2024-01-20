import { Options } from '../../types'
import { Field } from './base'

export const TimestampField = (options: Options = { primary: false, show: true }) => {
    options.type = 'Timestamp'
    return Field(options)
}
