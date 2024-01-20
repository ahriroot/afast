import { Model } from 'afast'
import { DateField, ForeignField, PrimaryField, StringField, TextField } from 'afast'
import { User } from './user'

export class Article extends Model {
    _table: string = 'article'

    @PrimaryField()
    id: number

    @StringField()
    title: string

    @TextField()
    content: string

    @DateField({ default: Date.now })
    created: Date

    @ForeignField({ foreign: User, references: 'id' })
    user: User
}
