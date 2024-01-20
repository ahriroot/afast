import { Model } from 'afast'
import { DateField, PrimaryField, StringField } from 'afast'

export class User extends Model {
    _table: string = 'user'

    @PrimaryField()
    id: number

    @StringField()
    name: string

    @StringField()
    username: string

    @StringField()
    password: string

    @DateField({ default: Date.now })
    created: Date
}
