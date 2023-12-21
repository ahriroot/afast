import DBPool from './db'

export type Dialect = {
    drop: (table: string) => string
    create: (table: string, fields: { type: string; name: string }[]) => string
}

declare global {
    namespace globalThis {
        var dialect: Dialect
        var pool: DBPool
        var wsc: any
    }
}

export {}
