import { Options } from '../../types'

const drop = (table: string) => {
    return `DROP TABLE IF EXISTS ${table}`
}

const create = (table: string, columns: Options[]) => {
    let sql = `CREATE TABLE IF NOT EXISTS ${table} (`
    let f
    columns.forEach((column) => {
        let def = ''
        if (column.default !== undefined) {
            if (typeof column.default === 'function') {
                def = ` DEFAULT ${column.default()}`
            } else {
                def = ` DEFAULT ${column.default}`
            }
        }
        switch (column.type) {
            case 'FieldPrimary':
                sql += `${column.name} SERIAL PRIMARY KEY, `
                break
            case 'FieldNumber':
                sql += `${column.name} INTEGER${def}, `
                break
            case 'FieldString':
                sql += `${column.name} VARCHAR(${column.length || 255})${def}, `
                break
            case 'FieldText':
                sql += `${column.name} TEXT${def}, `
                break
            case 'FieldBoolean':
                sql += `${column.name} BOOLEAN${def}, `
                break
            case 'FieldTimestamp':
                sql += `${column.name} TIMESTAMP${def}, `
                break
            case 'FieldForeign':
                f = new column.foreign()
                sql += `${column.name} INTEGER${def}, `
                sql += `foreign key(${column.name}) references ${f._table}(${column.references}), `
                break
            default:
                sql += `${column.name} INTEGER${def}, `
                break
        }
    })
    sql = sql.slice(0, -2)
    sql += `)`
    return sql
}

export default {
    drop,
    create,
}
