import { Serve, SocketAddress } from 'bun'
import { Model as M } from './model'

export declare class Model {
    _fields: { name: string; type: any; value: any }[] = []
    _stations: { name: string; value: any }[] = []

    _preload: (typeof Model)[] = []

    constructor(data: { [x: string]: any } = {})

    table(): string

    async save(): Promise<any>

    preload(models: (typeof Model)[]): Model

    exec(sql: string): any

    getFields(): {
        columns: {
            name: string
            type: any
            value: any
        }[]
        stations: {
            name: string
            value: any
        }[]
    }

    async migrate(client: DBPool, drop: boolean = false): Promise<any>

    async request_primary(primary: any): Promise<any>

    async request_get(
        page: number,
        size: number,
        sorts: string[] = []
    ): Promise<{
        count: number
        result: any
    }>

    async request_post(body: { [x: string]: any }): Promise<any>

    async request_put(primary: any, body: { [x: string]: any }): Promise<any>

    async request_delete(primary: any): Promise<any>
}

export declare class DBPool {
    db: any
    dialect: string
    constructor(dialect: string, database: any)

    // ======================= sqlite start =======================
    async exec_sqlite(sql: string): any

    async transaction_sqlite(sqls: string[]): any

    async query_sqlite(sql: string): any[]
    // ======================= sqlite end =======================

    // ======================= pg start =======================
    async exec_pg(sql: string): any

    async transaction_pg(sqls: string[]): any

    async query_pg(sql: string): any[]
    // ======================= pg end =======================

    async exec(sql: string): any

    async transaction(sqls: string[]): any

    async query(sql: string): any[]
}

export declare class Router {
    router: {
        [string: string]: { handler: Handler | Websocket; middlewares: Middleware[] }
    }
    views: { view: View; middlewares: Middleware[] } | undefined
    children: { [path: string]: Router }
    middlewares: Middleware[]

    /**
     * Http request method
     * @param method <string> request method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler | view | websocket
     * @param middlewares <afast.Middleware[]> middleware
     */
    method(
        method: string,
        paths: string[] | string,
        handler: Handler | View | Websocket,
        middlewares: Middleware[] = [],
        end = false
    ): void

    /**
     * Http request get method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    get(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []): void

    /**
     * Http request post method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    post(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []): void

    /**
     * Http request put method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    put(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []): void

    /**
     * Http request patch method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    patch(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []): void

    /**
     * Http request delete method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    delete(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []): void

    /**
     * Http request view (auto generate CRUD)
     * @param paths <string | string[]> request path
     * @param view <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    view(paths: string[] | string, view: View, middlewares: Middleware[] = []): void

    /**
     * Http request view (auto generate CRUD), auto reqister `${path}/:primary` router
     * @param paths <string | string[]> request path
     * @param view <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    viewId(paths: string[] | string, view: View, middlewares: Middleware[] = []): void

    /**
     * Http request websocket
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    ws(paths: string[] | string, websocket: Websocket, middlewares: Middleware[] = []): void

    /**
     * Create routing group
     * @param paths <string | string[]> request path
     * @param middlewares <afast.Middleware[]> middleware
     * @returns Router group
     */
    group(paths: string[] | string, middlewares: any[] = []): Router

    /**
     * Index the handler by path
     * @param paths <string[]> request path
     * @param params <{ [x: string]: any }> request params
     * @returns <afast.Router> router include handler and middlewares
     */
    async index(
        paths: string[],
        params: { [x: string]: any }
    ): Promise<{ router: Router; params: { [x: string]: any } } | undefined>
}

export type Options = {
    length?: number
    primary?: boolean
    show?: boolean
    default?: any
    nullable?: boolean
    foreign?: typeof M
    references?: string
}

type JsonResponse = { [key: string]: any } | null

export type Server = typeof Serve

/**
 * Configuration options for your application.
 * @property {string} [host] - The host address for the application.
 * @property {number} [port] - The port number for the application.
 * @property {boolean} [dev] - A flag indicating whether the application is in development mode.
 * @property {'sqlite' | 'pg' | 'mysql'} [dialect] - The database dialect for the application.
 * @property {any} [database] - Additional configuration options for the database.
 */
export type Config = {
    host?: string
    port?: number
    dev?: boolean
    static?: string
    dialect?: 'sqlite' | 'pg' | 'mysql'
    database?: any
    view_allowed?: string[]
    global?: any
}

export type WsClient = ServerWebSocket<unknown>

/**
 * Middleware for request and response
 * @interface Middleware
 * @exports Middleware
 * @property {function} request - Middleware for request before handler
 * @property {function} response - Middleware for response after handler
 */
export interface Middleware {
    /**
     * Middleware for request before handler
     * @param request <afast.ARequest> request
     * @param ws <afast.WsClient> websocket client (only for websocket)
     * @returns <afast.ARequest> request
     */
    request(request: ARequest, ws?: WsClient): Promise<ARequest | AResponse>

    /**
     * Middleware for response after handler
     * @param request <afast.ARequest> request
     * @param response <afast.AResponse> response
     * @returns <afast.AResponse> response
     */
    response(request: ARequest, response: AResponse): Promise<AResponse>
}

export type Handler = (request: ARequest, global?: typeof Config.global) => Promise<AResponse>

/**
 * View Handler
 * @interface View
 * @exports View
 * @property {afast.Model} model - DB Model
 * @property {string[]} allowed - Allowed http methods
 */
export interface View {
    model?: Model
    allowed?: string[]
}

/**
 * Websocket Handler
 * @interface Websocket
 * @exports Websocket
 * @property {function} open - Websocket open handler
 * @property {function} message - Websocket message handler
 * @property {function} close - Websocket close handler
 */
export interface Websocket {
    /**
     * Websocket open handler
     * @param ws <afast.WsClient> websocket client
     * @param request <afast.ARequest> request
     * @returns void
     */
    open(ws: AWebSocket, request: ARequest)

    /**
     * Websocket message handler
     * @param ws <afast.WsClient> websocket client
     * @param message <string> message
     * @returns void
     */
    message(ws: AWebSocket, message: string)

    /**
     * Websocket close handler
     * @param ws <afast.WsClient> websocket client
     * @returns void
     */
    close(ws: AWebSocket)
}

export type AResponse = JsonResponse | Response | Error

export declare const Default = {
    CURRENT_TIMESTAMP: 'CURRENT_TIMESTAMP',
}

/**
 * Migration database
 * @param config <afast.Config> config
 * @param models <afast.Model[]> models
 * @param drop <boolean> drop table
 * @returns <Promise<any[]>> sql execute results
 */
export declare function migrate(config?: Config, models: (typeof Model)[], drop: boolean = false) {}

/**
 * DB column primary type
 * @returns {FieldPrimary} Class FieldPrimary
 */
export declare function fieldPrimary() {}

/**
 * DB column number type
 * @param {object} option - Options
 * @param {boolean} option.show - Show in response
 * @param {number} option.default - Default value
 * @param {boolean} option.nullable - Nullable
 * @returns {FieldNumber} Class FieldNumber
 */
export declare function fieldNumber(option?: { show?: boolean; default?: number | null; nullable?: boolean }) {}

/**
 * DB column string type
 * @param {object} option - Options
 * @param {boolean} option.show - Show in response
 * @param {number} option.length - Max length
 * @param {string} option.default - Default value
 * @param {boolean} option.nullable - Nullable
 * @returns {FieldString} Class FieldString
 */
export declare function fieldString(options?: {
    show?: boolean
    length?: number
    default?: string | null
    nullable?: boolean
}) {}

/**
 * DB column timestamp type
 * @param {object} option - Options
 * @param {boolean} option.show - Show in response
 * @param {number} option.default - Default value
 * @param {boolean} option.nullable - Nullable
 * @returns {FieldTimestamp} Class FieldTimestamp
 */
export declare function fieldTimestamp(options?: {
    show?: boolean
    default?: number | string | null
    nullable?: boolean
}) {}

/**
 * DB column foreign key type
 * @param {object} option - Options
 * @param {boolean} option.show - Show in response
 * @param {boolean} option.nullable - Nullable
 * @param {typeof Model} option.foreign - Foreign model
 * @param {string} option.references - References column
 * @returns {FieldForeign} Class FieldForeign
 */
export declare function fieldForeign(options?: {
    show?: boolean
    nullable?: boolean
    foreign: typeof Model
    references?: string
}) {}

/**
 * Database base Model
 * @class Model
 * @exports Model
 * @property {string} table - Table name
 * @property {function} migrate - Migrate database
 * @property {function} getFields - Get fields
 * @property {function} request_primary - Request primary (CRUD select by primary)
 * @property {function} request_get - Request get (CRUD select)
 * @property {function} request_post - Request post (CRUD insert)
 * @property {function} request_put - Request put (CRUD update)
 * @property {function} request_delete - Request delete (CRUD delete)
 */
export declare class Model {
    _fields: { name: string; type: any; value: any }[]
    _stations: { name: string; value: any }[]
    constructor()

    /**
     * Table name
     * @returns {string} table name
     */
    table(): string

    /**
     * Migrate database
     * @param {afast.DBPool} client - Database client
     * @param {boolean} drop - Drop table
     * @returns {Promise<any>} sql execute result
     */
    migrate(client: DBPool, drop?: boolean): Promise<any>

    /**
     * Get table columns
     * @returns {{name: string;type: any;value: any}[]} Table columns
     */
    getFields(): {
        columns: {
            name: string
            type: any
            value: any
        }[]
        stations: {
            name: string
            value: any
        }[]
    }

    /**
     * Request primary (CRUD select by primary)
     * @param {any} primary - Primary value
     * @returns {Promise<any>} Data
     */
    request_primary(primary: any): Promise<any>

    /**
     * Request get (CRUD select)
     * @param {number} page - Page number
     * @param {number} size - Page size
     * @param {string[]} sorts - Sorts
     * @returns {Promise<any>} Data
     */
    request_get(page: number, size: number, sorts?: string[]): Promise<any>

    /**
     * Request post (CRUD insert)
     * @param {object} body - Request body
     * @returns {Promise<any>} Result of insert
     */
    request_post(body: { [x: string]: any }): Promise<any>

    /**
     * Request put (CRUD update)
     * @param {any} primary - Primary value
     * @param {object} body - Request body
     * @returns {Promise<any>} Result of update
     */
    request_put(primary: any, body: { [x: string]: any }): Promise<any>

    /**
     * Request delete (CRUD delete)
     * @param {any} primary - Primary value
     * @returns {Promise<any>} Result of delete
     */
    request_delete(primary: any): Promise<any>
}

/**
 * Http request
 * @class ARequest
 * @exports ARequest
 * @property {Request} request - Request
 * @property {string} method - Http method
 * @property {URL} url - Http url
 * @property {{ [x: string]: any }} headers - Http headers
 * @property {{ [x: string]: any }} query - Http query
 * @property {{ [x: string]: any }} params - Http params
 * @property {{ [x: string]: any }} local - Local value
 * @property {any} body - Http body
 */
export declare class ARequest {
    request: Request
    method: string
    url: URL
    headers: { [x: string]: any }
    query: { [x: string]: any }
    params: { [x: string]: any }
    local: { [x: string]: any } = {}
    body: any

    /**
     * @param {Request} request - Request
     * @param {{ [x: string]: any }} params - Params
     */
    constructor(request: Request, params: { [x: string]: any })

    /**
     * Parse request body
     * @param {Request} request - Request
     * @param {{ [x: string]: any }} params - Params
     * @returns {Promise<void>} void
     */
    static async parse(request: Request, params?: { [x: string]: any } = {}): Promise<ARequest>

    /**
     * Parse request body
     * @returns {Promise<void>} void
     */
    async parseBody(): Promise<void>

    /**
     * Set params
     * @param {{ [x: string]: any }} params - Params
     * @returns {void} void
     */
    setParams(params: { [x: string]: any }): void

    /**
     * Set local value
     * @param {string} key - Key
     * @param {any} value - Value
     * @returns {void} void
     */
    set(key: string, value: any): void

    /**
     * Get local value
     * @param {string} key - Key
     * @returns {any} Value
     */
    get(key: string): any
}

export type HTTPServer = {
    address: SocketAddress
    development: boolean
    fetch(this: Server, request: Request, server: Server): Response | Promise<Response>
    hostname: string
    id: string
    pendingRequests: number
    pendingWebSockets: number
    port: number
    protocol: string
    publish(
        topic: string,
        data: string | ArrayBufferView | ArrayBuffer | SharedArrayBuffer,
        compress?: boolean
    ): ServerWebSocketSendStatus
    reload(options: Serve): void
    requestIP(request: Request): SocketAddress | null
    stop(closeActiveConnections?: boolean): void
    upgrade<T = undefined>(
        request: Request,
        options?: {
            /**
             * Send any additional headers while upgrading, like cookies
             */
            headers?: HeadersInit
            /**
             * This value is passed to the {@link ServerWebSocket.data} property
             */
            data?: T
        }
    ): boolean
    url: URL
}

/**
 * AFast App
 * @class Response
 * @exports Response
 */
export declare class App {
    constructor()

    /**
     * Get router map
     * @returns {string} Tree of router map
     */
    map(): string

    /**
     * Get router map json
     * @returns {any} Tree of router map format json
     */
    mapJson(): any

    /**
     * Register middleware
     * @param {afast.Middleware} middleware - Middleware
     * @returns {void} void
     */
    use(middleware: Middleware): void

    /**
     * Register http get handler
     * @param {string} path - Path
     * @param {function} handler - Handler
     * @param {afast.Middleware[]} middleware - Middleware
     */
    get(path: string, handler: Handler, middleware?: Middleware[]): void

    /**
     * Register http post handler
     * @param {string} path - Path
     * @param {function} handler - Handler
     * @param {afast.Middleware[]} middleware - Middleware
     */
    post(path: string, handler: Handler, middleware?: Middleware[]): void

    /**
     * Register http put handler
     * @param {string} path - Path
     * @param {function} handler - Handler
     * @param {afast.Middleware[]} middleware - Middleware
     */
    put(path: string, handler: Handler, middleware?: Middleware[]): void

    /**
     * Register http patch handler
     * @param {string} path - Path
     * @param {function} handler - Handler
     * @param {afast.Middleware[]} middleware - Middleware
     */
    patch(path: string, handler: Handler, middleware?: Middleware[]): void

    /**
     * Register http delete handler
     * @param {string} path - Path
     * @param {function} handler - Handler
     * @param {afast.Middleware[]} middleware - Middleware
     */
    delete(path: string, handler: Handler, middleware?: Middleware[]): void

    /**
     * Http request view (auto generate CRUD)
     * @param {string} path - Path
     * @param {afast.View} view - View
     * @param {afast.Middleware[]} middleware - Middleware
     */
    view(path: string, view: View, middleware?: Middleware[]): void

    /**
     * Http request view (auto generate CRUD), auto reqister `${path}/:primary` router
     * @param {string} path - Path
     * @param {afast.View} view - View
     * @param {afast.Middleware[]} middleware - Middleware
     */
    viewId(path: string, view: View, middleware?: Middleware[]): void

    /**
     * Register websocket handler
     * @param {string} path - Path
     * @param {afast.Websocket} websocket - Websocket
     * @param {afast.Middleware[]} middleware - Middleware
     */
    ws(path: string, websocket: Websocket, middleware?: Middleware[]): void

    /**
     * Create routing group
     * @param {string} path - Path
     * @param {afast.Router} router - Router
     * @param {afast.Middleware[]} middleware - Middleware
     * @returns {afast.Router} Router
     */
    group(path: string, middleware?: Middleware[]): Router

    /**
     * @param {afast.Config { port?: number, host?: string, dev?: boolean, dialect?: 'sqlite' | 'pg' | 'mysql, database?: any}} config - Config
     * @returns {Bun.Server} server
     * @description Run http server
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
     * const server = app.run(config)
     * ```
     */
    run(config?: Config): HTTPServer
}

export * from './middleware/types'
