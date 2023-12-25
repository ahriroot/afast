import DBPool from './db'

export * from './model/primary'
export * from './model/number'
export * from './model/float'
export * from './model/string'
export * from './model/text'
export * from './model/boolean'
export * from './model/timestamp'
export * from './model/json'

export const Default = {
    CURRENT_TIMESTAMP: 'CURRENT_TIMESTAMP',
}

export type Options = {
    length?: number
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
            return await client.transaction([`DROP TABLE IF EXISTS ${this.table()}`, sql])
        }
        return await client.exec(sql)
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
        const res = await global.pool.query(sql)
        return res.length > 0 ? res[0] : null
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
            const res = await global.pool.query(sql)

            const countSql = `SELECT COUNT(*) AS count FROM ${this.table()}`
            const countRes = await global.pool.query(countSql)

            return {
                count: countRes[0].count,
                result: res,
            }
        } else {
            const sql = `SELECT ${columns.join(', ')} FROM ${this.table()} LIMIT ${limit} OFFSET ${offset}`
            const res = await global.pool.query(sql)

            const countSql = `SELECT COUNT(*) AS count FROM ${this.table()}`
            const countRes = await global.pool.query(countSql)

            return {
                count: countRes[0].count,
                result: res,
            }
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

        const res = await global.pool.exec(sql)

        return res
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

        const res = await global.pool.exec(sql)

        return res
    }

    async request_delete(primary: any) {
        const sql = `DELETE FROM ${this.table()} WHERE id = ${primary}`

        const res = await global.pool.exec(sql)

        return res
    }
}

export class Field {
    length?: number
    show: boolean = true
    primary: boolean = false
    default?: any
    nullable?: boolean = false
    constructor(options: Options) {
        if (options.length !== undefined) {
            this.length = options.length
        }
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
