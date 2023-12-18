import { Pool, PoolClient } from 'pg'
import DBPool from './db'

export const Default = {
    CURRENT_TIMESTAMP: 'CURRENT_TIMESTAMP',
}

export type Options = {
    primary?: boolean
    show?: boolean
    default?: any
    nullable?: boolean
}

export class Model {
    private _fields?: { name: string; type: any; value: any }[] = undefined

    table() {
        const name = this.constructor.name
        const table = name.replace(/([A-Z])/g, '_$1').toLowerCase()
        if (table[0] === '_') {
            return table.slice(1)
        }
        return table
    }

    async conn() {
        const pool = new Pool({
            user: 'postgres',
            host: '127.0.0.1',
            database: 'afast',
            password: 'Aa12345.',
            port: 5432,
        })
        const client = pool.connect()
        return client
    }

    getFields() {
        if (this._fields) {
            return this._fields
        }
        const fields = Object.getOwnPropertyDescriptors(this)
        const columns = []
        for (const key in fields) {
            if (fields[key].value instanceof Field) {
                columns.push({
                    name: key,
                    type: fields[key].value.constructor.name,
                    value: fields[key].value,
                })
            }
        }
        this._fields = columns
        return columns
    }

    async migrate(client: DBPool, drop: boolean = false) {
        const columns = this.getFields()
        const sql = global.dialect.create(this.table(), columns)
        if (drop) {
            await client.transaction([`DROP TABLE IF EXISTS ${this.table()}`, sql])
        }
        await client.exec(sql)
    }

    async request_primary(primary: any) {
        const columns = this.getFields()
            .filter((column) => {
                return column.value.show
            })
            .map((column) => {
                return column.name
            })
        const sql = `SELECT ${columns.join(', ')} FROM ${this.table()} WHERE id = ${primary}`
        return sql
    }

    async request_get(page: number, size: number, sorts: string[] = []) {
        const fields = this.getFields()
        const columns = fields
            .filter((column) => {
                return column.value.show
            })
            .map((column) => {
                return column.name
            })
        const limit = size
        const offset = (page - 1) * size
        if (sorts.length > 0) {
            const allColumns = fields
                .filter((column) => {
                    return column.value.show
                })
                .map((column) => {
                    return column.name
                })
            const sortColumns: string[] = []
            sorts.forEach((sort) => {
                const s = sort[0] === '-' ? sort.slice(1) : sort
                if (allColumns.includes(s)) {
                    sortColumns.push(`${s} ${sort[0] === '-' ? 'DESC' : 'ASC'}`)
                }
            })
            const sql = `SELECT ${columns.join(', ')} FROM ${this.table()} ORDER BY ${sortColumns.join(
                ', '
            )} LIMIT ${limit} OFFSET ${offset}`
            return sql
        } else {
            const sql = `SELECT ${columns.join(', ')} FROM ${this.table()} LIMIT ${limit} OFFSET ${offset}`
            return sql
        }
    }

    async request_post(body: { [x: string]: any }) {
        const columns: string[] = []
        const values: string[] = []
        this.getFields().forEach((field) => {
            if (!field.value.primary) {
                columns.push(field.name)
                switch (field.type) {
                    case 'FieldNumber':
                        values.push(body[field.name])
                        break
                    case 'FieldString':
                        values.push(`'${body[field.name]}'`)
                        break
                    case 'FieldTimestamp':
                        values.push(`'${body[field.name]}'`)
                        break
                    default:
                        values.push(body[field.name])
                        break
                }
            }
        })
        const sql = `INSERT INTO ${this.table()} (${columns.join(', ')}) VALUES (${values.join(', ')})`
        return sql
    }

    async request_put(primary: any, body: { [x: string]: any }) {
        const columns: string[] = []
        const keys = Object.keys(body)
        this.getFields().forEach((field) => {
            if (!field.value.primary && keys.includes(field.name)) {
                switch (field.type) {
                    case 'FieldNumber':
                        columns.push(`${field.name} = ${body[field.name]}`)
                        break
                    case 'FieldString':
                        columns.push(`${field.name} = '${body[field.name]}'`)
                        break
                    case 'FieldTimestamp':
                        columns.push(`${field.name} = '${body[field.name]}'`)
                        break
                    default:
                        columns.push(`${field.name} = ${body[field.name]}`)
                        break
                }
            }
        })
        const sql = `UPDATE ${this.table()} SET ${columns.join(', ')} WHERE id = ${primary}`
        return sql
    }

    async request_delete(primary: any) {
        const sql = `DELETE FROM ${this.table()} WHERE id = ${primary}`
        return sql
    }
}

export class Field {
    show: boolean = true
    primary: boolean = false
    default?: any
    nullable?: boolean = false
    constructor(options: Options) {
        if (options.show !== undefined) {
            this.show = options.show
        }
        if (options.primary !== undefined) {
            this.primary = options.primary
        }
        this.default = options.default
        if (options.nullable !== undefined) {
            this.nullable = options.nullable
        }
    }
}

export class FieldPrimary extends Field {
    constructor(options: Options) {
        options.primary = true
        super(options)
    }
}

export const fieldPrimary = (options: Options = {}) => {
    return new FieldPrimary(options)
}

export class FieldNumber extends Field {
    constructor(options: Options) {
        options.primary = false
        super(options)
    }
}

export const fieldNumber = (options: Options = {}) => {
    return new FieldNumber(options)
}

export class FieldString extends Field {
    constructor(options: Options) {
        options.primary = false
        super(options)
    }
}

export const fieldString = (options: Options = {}) => {
    return new FieldString(options)
}

export class FieldText extends Field {
    constructor(options: Options) {
        options.primary = false
        super(options)
    }
}

export const fieldText = (options: Options = {}) => {
    return new FieldText(options)
}

export class FieldBoolean extends Field {
    constructor(options: Options) {
        options.primary = false
        super(options)
    }
}

export const fieldBoolean = (options: Options = {}) => {
    return new FieldBoolean(options)
}

export class FieldTimestamp extends Field {
    constructor(options: Options) {
        options.primary = false
        super(options)
    }
}

export const fieldTimestamp = (options: Options = {}) => {
    return new FieldTimestamp(options)
}
