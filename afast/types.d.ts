import { Serve, SocketAddress } from 'bun'
import { App, Router } from '.'
import { Model } from './model'

export type Options = {
    length?: number
    primary?: boolean
    show?: boolean
    default?: any
    nullable?: boolean
}

type JsonResponse = { [key: string]: any }

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
    dialect?: 'sqlite' | 'pg' | 'mysql'
    database?: any
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
    request(request: ARequest, ws?: WsClient): Promise<ARequest>

    /**
     * Middleware for response after handler
     * @param request <afast.ARequest> request
     * @param response <afast.AResponse> response
     * @returns <afast.AResponse> response
     */
    response(request: ARequest, response: AResponse): Promise<AResponse>
}

export type Handler = (request: ARequest) => Promise<AResponse>

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

export type AResponse = JsonResponse | Response

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
export declare function migrate(config?: Config, models: (typeof Model)[], drop: boolean) {}

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
        name: string
        type: any
        value: any
    }[]

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
    static async parse(request: Request, params: { [x: string]: any }): Promise<ARequest>

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
     * @returns {Router} Tree of router map
     */
    map(): string

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
