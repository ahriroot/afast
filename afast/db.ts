class DBPool {
    db: any
    dialect: string
    constructor(dialect: string, database: any) {
        this.dialect = dialect
        switch (dialect) {
            case 'sqlite':
                const { Database } = require('bun:sqlite')
                this.db = new Database(database.path)
                break
            case 'pg':
                const { Pool } = require('pg')
                this.db = new Pool({
                    host: database.host,
                    port: database.port,
                    user: database.user,
                    password: database.pass,
                    database: database.name,
                })
                break
        }
    }

    // ======================= sqlite start =======================
    async exec_sqlite(sql: string) {
        return await this.db.exec(sql)
    }

    async transaction_sqlite(sqls: string[]): Promise<number> {
        const tx = this.db.transaction((ss: string[]) => {
            for (const sql of ss) {
                this.db.exec(sql)
            }
            return ss.length
        })
        const count = tx(sqls)
        return count
    }

    async query_sqlite(sql: string) {
        return await this.db.prepare(sql).all()
    }
    // ======================= sqlite end =======================

    // ======================= pg start =======================
    async exec_pg(sql: string) {
        const client = await this.db.connect()
        const res = await client.query(sql)
        client.release()
        return res
    }

    async transaction_pg(sqls: string[]) {
        const client = await this.db.connect()
        await client.query('BEGIN')
        try {
            const results: any[] = []
            for (const sql of sqls) {
                results.push(await client.query(sql))
            }
            await client.query('COMMIT')
            return results
        } catch (e) {
            await client.query('ROLLBACK')
            throw e
        } finally {
            client.release()
        }
    }

    async query_pg(sql: string) {
        const client = await this.db.connect()
        const res = await client.query(sql)
        client.release()
        return res.rows
    }
    // ======================= pg end =======================

    async exec(sql: string) {
        switch (this.dialect) {
            case 'sqlite':
                return await this.exec_sqlite(sql)
            case 'pg':
                return await this.exec_pg(sql)
        }
    }

    async transaction(sqls: string[]) {
        switch (this.dialect) {
            case 'sqlite':
                return await this.transaction_sqlite(sqls)
            case 'pg':
                return await this.transaction_pg(sqls)
        }
    }

    async query(sql: string) {
        switch (this.dialect) {
            case 'sqlite':
                return await this.query_sqlite(sql)
            case 'pg':
                return await this.query_pg(sql)
        }
    }
}

export default DBPool
