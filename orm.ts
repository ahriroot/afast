import 'reflect-metadata'

const Field = (options: { primary?: boolean; name?: string; default?: any } = { primary: false }) => {
    return (target: any, propertyKey: any) => {
        let value: any = target[propertyKey]

        if (!['String', 'Number', 'Date'].includes(target.constructor.name)) {
            let models = Reflect.getMetadata('models', global) || {}
            if (!Object.keys(models).includes(target.constructor.name)) {
                models[target.constructor.name] = target
                Reflect.defineMetadata('models', models, global)
            }
        }

        let type = Reflect.getMetadata('design:type', target, propertyKey).name
        let fields = Reflect.getMetadata('fields', target) || []
        fields.push({
            ...options,
            name: options.name || propertyKey,
            property: propertyKey,
            type: type,
        })

        Reflect.defineMetadata('fields', fields, target)

        const getter = () => value
        const setter = (newValue: string) => {
            value = newValue
        }

        Object.defineProperty(target, propertyKey, {
            get: getter,
            set: setter,
            enumerable: true,
            configurable: true,
        })
    }
}

const PrimaryField = (options: { name?: string } = { name: 'id' }) => {
    return Field({ primary: true, name: options.name })
}

class Model {
    _table?: string
    constructor() {
        this._table = this.constructor.name.toLowerCase()
    }
    primaryKey() {
        const fieldsOptions = Reflect.getMetadata('fields', this) || []
        const primary = fieldsOptions.find((field: any) => field.primary)
        return primary
    }
    static preload() {
        return new this()
    }
    async save() {
        const tablename = this._table
        const fieldsOptions = Reflect.getMetadata('fields', this) || []
        const primary = fieldsOptions.find((field: any) => field.primary)
        const primaryValue = this[primary.property]
        const fields = fieldsOptions
            .filter((field: any) => !field.primary)
            .map((field: any) => field.name)
            .join(',')

        const kv: any = {}
        for (const field of fieldsOptions) {
            if (!field.primary) {
                let value = this[field.property]
                if (value === undefined) {
                    if (typeof field.default === 'function') {
                        value = field.default()
                    } else {
                        value = field.default || null
                    }
                } else {
                    if (primaryValue !== undefined && primaryValue !== null && typeof value.save === 'function') {
                        await value.save()
                        value = value[value.primaryKey().property]
                    }
                }

                if (value === null) {
                    kv[field.name] = null
                } else {
                    switch (field.type) {
                        case 'String':
                            kv[field.name] = `'${value}'`
                            break
                        case 'Number':
                            kv[field.name] = value
                            break
                        case 'Date':
                            kv[field.name] = `'${value}'`
                            break
                        default:
                            kv[field.name] = value
                            break
                    }
                }
            }
        }

        if (primaryValue === undefined || primaryValue === null) {
            const sql = `INSERT INTO ${tablename} (${fields}) VALUES (${Object.values(kv).join(', ')})`
            console.log(sql)
        } else {
            const sql = `UPDATE ${tablename} SET ${Object.entries(kv)
                .map(([k, v]) => `${k}=${v}`)
                .join(', ')} WHERE ${primary.name}=${primaryValue}`
            console.log(sql)
        }
    }
}

const now = () => {
    return new Date().getTime()
}

const nowDate = () => {
    // 2022-01-08 12:34:56
    return new Date().toISOString().replace('T', ' ').replace(/\..+/, '')
}

class User extends Model {
    _table = 'users'

    @PrimaryField()
    primary: number

    @Field()
    username: string

    @Field()
    password: string

    @Field({ default: nowDate })
    last_login: Date

    @Field({ default: now })
    created: number

    @Field({ default: now })
    updated: number
}

class Article extends Model {
    _table = 'articles'

    @PrimaryField()
    id: number

    @Field()
    title: string

    @Field()
    content: string

    @Field()
    user: User
}

const main = async () => {
    const user = new User()
    user.primary = 1
    user.username = '123'
    // console.log(await user.save())

    const article = new Article()
    article.user = user
    article.id = 1
    console.log(article.user)
    await article.save()
}

main()
