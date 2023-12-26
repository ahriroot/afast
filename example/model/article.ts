import { Default, Model, fieldPrimary, fieldString, fieldTimestamp, fieldForeign } from 'afast'
import { UserModel } from './user'

export class ArticleModel extends Model {
    table() {
        return 'article'
    }
    id = fieldPrimary()
    title = fieldString()
    content = fieldString()
    created = fieldTimestamp({ default: Default.CURRENT_TIMESTAMP })
    user = fieldForeign({ foreign: UserModel, references: 'id' })
}
