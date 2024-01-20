import 'reflect-metadata'
import { Options } from '../../types'

export { PrimaryField } from './primary'
export { StringField } from './string'
export { NumberField } from './number'
export { FloatField } from './float'
export { DateField } from './date'
export { TextField } from './text'
export { BooleanField } from './boolean'
export { TimestampField } from './timestamp'
export { ForeignField } from './foreign'

class Model {
    _table: string
    _preload: (typeof Model)[] = []
    _where: { [x: string]: any } = {}
    _filter: any[] = []
    _limit: number = 10
    _offset: number = 0
    _fields: Options[] = []
    _sorts: string[] = []
    constructor(data?: any) {
        this._table = this.constructor.name.toLowerCase()
        if (data !== undefined) {
            const fieldsOptions = this.fields()
            const fields = fieldsOptions.map((field: any) => field.property) as (keyof this)[]
            const keys = Object.keys(data)
            for (const field of fields) {
                if (keys.includes(field as string)) {
                    this[field] = data[field]
                }
            }
        }
    }
    async serialize() {
        const fieldsOptions = this.fields()
        const self = this as any
        const keys = fieldsOptions.map((field: any) => field.property)
        let data: any = {}
        for (let i = 0; i < keys.length; i++) {
            data[keys[i]] = self[keys[i]]
        }
        return data
    }
    static deserialize(data: { [x: string]: any }) {
        const m = new this()
        const fields = m.fields().map((field: any) => field.property)
        const keys = Object.keys(data)
        const self = m as any
        for (const field of fields) {
            if (keys.includes(field)) {
                self[field] = data[field]
            }
        }
        return m
    }
    fields() {
        if (this._fields.length === 0) {
            this._fields = Reflect.getMetadata('fields', this) || []
        }
        return this._fields
    }
    primaryKey() {
        const fieldsOptions = this.fields()
        const primary = fieldsOptions.find((field: any) => field.primary)
        return primary
    }
    async migrate(client: any, drop: boolean = false) {
        const fieldsOptions = Reflect.getMetadata('fields', this) || []
        const sql = global.dialect.create(this._table, fieldsOptions)
        if (drop) {
            return await client.transaction([`DROP TABLE IF EXISTS ${this._table}`, sql])
        }
        return await client.exec(sql)
    }

    _save() {
        const self = this as any
        const sqls: string[] = []
        const tablename = this._table
        const fieldsOptions = Reflect.getMetadata('fields', this) || []
        const primary = fieldsOptions.find((field: any) => field.primary)
        const primaryValue = self[primary.property]
        const fields = fieldsOptions
            .filter((field: any) => !field.primary)
            .map((field: any) => field.name)
            .join(',')

        const kv: any = {}
        for (const field of fieldsOptions) {
            if (!field.primary) {
                let value = self[field.property]
                if (value === undefined) {
                    if (typeof field.default === 'function') {
                        value = field.default()
                    } else {
                        value = field.default || null
                    }
                } else {
                    if (primaryValue !== undefined && primaryValue !== null && typeof value.save === 'function') {
                        sqls.push(...value._save())
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
                            if (value instanceof Model) {
                                value = value as any
                                kv[field.name] = value[value.primaryKey().property]
                            } else {
                                kv[field.name] = value
                            }
                            break
                    }
                }
            }
        }

        if (primaryValue === undefined || primaryValue === null) {
            sqls.push(`INSERT INTO ${tablename} (${fields}) VALUES (${Object.values(kv).join(', ')})`)
        } else {
            sqls.push(
                `UPDATE ${tablename} SET ${Object.entries(kv)
                    .map(([k, v]) => `${k}=${v}`)
                    .join(', ')} WHERE ${primary.name}=${primaryValue}`
            )
        }

        return sqls
    }

    static preload(...args: (typeof Model)[]) {
        const self = new this()
        self._preload = args
        return self
    }
    async save() {
        const sqls = this._save()
        if (sqls.length === 1) {
            return await global.pool.exec(sqls[0])
        } else {
            return await global.pool.transaction(sqls)
        }
    }
    async delete() {
        const self = this as any
        const tablename = this._table
        const fieldsOptions = Reflect.getMetadata('fields', this) || []
        const primary = fieldsOptions.find((field: any) => field.primary)
        const primaryValue = self[primary.property]

        if (primaryValue === undefined && primaryValue === null) {
            throw new Error('primary key is null')
        }
        return await global.pool.exec(`DELETE FROM ${tablename} WHERE ${primary.name}=${primaryValue}`)
    }
    static where(kv: { [x: string]: any }) {
        const self = new this()
        self.where(kv)
        return self
    }
    where(kv: { [x: string]: any }) {
        this._where = { ...this._where, ...kv }
        return this
    }
    static filter(kv: any[]) {
        const self = new this()
        self.filter(kv)
        return self
    }
    filter(kv: any[]) {
        this._filter = [...this._filter, ...kv]
        return this
    }
    static paginate(page: number, size: number) {
        const self = new this()
        self.paginate(page, size)
        return self
    }
    paginate(page: number, size: number) {
        this._limit = size
        this._offset = (page - 1) * this._limit
        return this
    }
    static page(page: number) {
        const self = new this()
        self.page(page)
        return self
    }
    page(page: number) {
        this._offset = (page - 1) * this._limit
        return this
    }
    static size(size: number) {
        const self = new this()
        self.size(size)
        return self
    }
    size(size: number) {
        const old = this._limit
        this._limit = size
        this._offset = (this._offset / old) * this._limit
        return this
    }
    sort(sorts: string[]) {
        this._sorts = sorts
        return this
    }
    set(data: { [x: string]: any }) {
        const self = this as any
        const fieldsOptions = Reflect.getMetadata('fields', this) || []
        const keys = Object.keys(data)
        for (const field of fieldsOptions) {
            if (keys.includes(field.property)) {
                self[field.property] = data[field.property]
            }
        }
        return this
    }
    static async select() {
        const self = new this()
        return self.select()
    }
    async first() {
        const result = await this.size(1).select()
        if (result === null || result.length < 1) {
            return null
        }
        return result[0]
    }
    async select() {
        const self = this as any
        const sqls: string[] = []
        const tablename = this._table
        const fieldsOptions = Reflect.getMetadata('fields', this) || []

        const kv: string[] = []
        for (const field of fieldsOptions) {
            if (field.show) {
                if (field.foreign !== undefined) {
                    const m = new field.foreign()
                    const fk = self[field.property][m.primaryKey().property]
                    if (fk === undefined || fk === null) {
                        if (this._filter.includes(field.property)) {
                            kv.push(field.name)
                        }
                    } else {
                        const filter = this._filter
                            .filter((f) => f.includes(`${field.property}.`))
                            .map((f) => {
                                return f.split('.').slice(1).join('.')
                            })
                        self[field.property] = await m
                            .where({
                                [field.references]: fk,
                            })
                            .filter(filter)
                            .select()
                    }
                } else {
                    if (this._filter.length === 0) {
                        kv.push(field.name)
                    } else if (this._filter.includes(field.property)) {
                        kv.push(field.name)
                    }
                }
            }
        }

        let orderBys = this._sorts.map((sort) => {
            if (sort.startsWith('-')) {
                return `${sort.slice(1)} DESC`
            } else {
                return `${sort} ASC`
            }
        })

        let orderBy = ''
        if (orderBys.length > 0) {
            orderBy = ` ORDER BY ${orderBys.join(', ')}`
        }

        if (Object.keys(this._where).length === 0) {
            sqls.push(`SELECT ${kv.join(',')} FROM ${tablename}${orderBy} LIMIT ${this._limit} OFFSET ${this._offset}`)
        } else {
            const where: string[] = []
            for (const [k, v] of Object.entries(this._where)) {
                if (v === null) {
                    where.push(`${k} IS NULL`)
                } else if (typeof v === 'string') {
                    where.push(`${k}='${v}'`)
                } else {
                    where.push(`${k}=${v}`)
                }
            }

            sqls.push(
                `SELECT ${kv.join(',')} FROM ${tablename} WHERE ${where.join(' AND ')} LIMIT ${this._limit} OFFSET ${
                    this._offset
                }`
            )
        }

        if (sqls.length > 0) {
            const rows = await global.pool.query(sqls[0])
            const result: any[] = []
            for (let i = 0; i < rows.length; i++) {
                result.push(new (this.constructor as { new (data?: any): any })(rows[i]))
            }
            return result
        }

        return null
    }

    async count() {
        const self = this as any
        const sqls: string[] = []
        const tablename = this._table

        if (Object.keys(this._where).length === 0) {
            sqls.push(`SELECT COUNT(*) AS count FROM ${tablename}`)
        } else {
            const where: string[] = []
            for (const [k, v] of Object.entries(this._where)) {
                if (v === null) {
                    where.push(`${k} IS NULL`)
                } else if (typeof v === 'string') {
                    where.push(`${k}='${v}'`)
                } else {
                    where.push(`${k}=${v}`)
                }
            }

            sqls.push(`SELECT COUNT(*) AS count FROM ${tablename} WHERE ${where.join(' AND ')}`)
        }

        if (sqls.length > 0) {
            const rows = await global.pool.query(sqls[0])
            if (rows.length > 0) {
                return rows[0].count
            }
        }

        return null
    }
}

export { Model }
