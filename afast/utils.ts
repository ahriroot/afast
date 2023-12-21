import DBPool from "./db"
import { Model } from "./model"
import { Config } from "./types"

/**
 * @param config <afast.Config { port?: number, host?: string, dev?: boolean, dialect?: 'sqlite' | 'pg' | 'mysql, database?: any}> config
 * @param models <afast.Model[]> models
 * @param drop <boolean> drop table before migrate
 * @returns <Promise<any[]>> sql execute results
 * @description Migrate database
 * ```typescript
 * const config: Config = {
 *    port: 3000,
 *    host: 'localhost',
 *    dialect: 'sqlite', // sqlite pg mysql
 *    database: {
 *        path: './test.db',
 *    },
 *    // database: {
 *        // host: '127.0.0.1',
 *        // port: 5432,
 *        // user: 'postgres',
 *        // pass: '',
 *        // name: 'afast',
 *    // },
 * }
 *
 * await migrate(config, [TestModel], true)
 * ```
 */
export const migrate = async (config: Config, models: (typeof Model)[], drop: boolean = false): Promise<any[]> => {
    if (config.dialect && ['sqlite', 'pg'].includes(config.dialect)) {
        global.dialect = require(`./dialect/${config.dialect}`).default
        if (config.database) {
            global.pool = new DBPool(config.dialect, config.database)
        }
    }

    const results: any[] = []

    for (const model of models) {
        const obj = new model()
        results.push(await obj.migrate(global.pool, drop))
    }

    return results
}
