import { Default, Model, fieldNumber, fieldPrimary, fieldString, fieldTimestamp } from "afast"

export class UserModel extends Model {
    table() {
        return 'user'
    }
    id = fieldPrimary()
    name = fieldString()
    age = fieldNumber({ default: 0 })
    username = fieldString()
    password = fieldString({ show: false })
    created = fieldTimestamp({ default: Default.CURRENT_TIMESTAMP })
}
