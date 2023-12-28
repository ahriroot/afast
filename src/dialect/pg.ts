const drop = (table: string) => {
    return `DROP TABLE IF EXISTS ${table}`
}

const create = (table: string, columns: { type: string; name: string; value: any }[]) => {
    let sql = `CREATE TABLE IF NOT EXISTS ${table} (`
    let f
    columns.forEach((column) => {
        let def = ''
        if (column.value.default !== undefined) {
            def = ` DEFAULT ${column.value.default}`
        }
        switch (column.type) {
            case 'FieldPrimary':
                sql += `${column.name} SERIAL PRIMARY KEY, `
                break
            case 'FieldNumber':
                sql += `${column.name} INTEGER${def}, `
                break
            case 'FieldString':
                sql += `${column.name} VARCHAR(${column.value.length || 255})${def}, `
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
                f = new column.value.foreign()
                sql += `${column.name} INTEGER${def}, `
                sql += `foreign key(${column.name}) references ${f.table()}(${column.value.references}), `
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
