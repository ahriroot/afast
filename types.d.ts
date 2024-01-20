import { Serve, SocketAddress } from 'bun'

export { App } from './src/app'
export { ARequest } from './src/request'
export {
    Model,
    PrimaryField,
    StringField,
    NumberField,
    DateField,
    TextField,
    BooleanField,
    TimestampField,
    ForeignField,
} from './src/model'
export { DBPool } from './src/db'
export { Router } from './src/router'
export { cors } from './src/middleware/cors'

export type Dialect = {
    drop: (table: string) => string
    create: (table: string, fields: { type: string; name: string }[]) => string
}

declare global {
    namespace globalThis {
        var dialect: Dialect
        var pool: DBPool
    }
}

export type Options = {
    primary?: boolean
    name?: string = ''
    property?: string = ''
    default?: any
    foreign?: typeof Model
    references?: string
    show?: boolean
    type?: string
    length?: number
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
    request(request: ARequest, ws?: WsClient, global?: any): Promise<ARequest | AResponse>

    /**
     * Middleware for response after handler
     * @param request <afast.ARequest> request
     * @param response <afast.AResponse> response
     * @returns <afast.AResponse> response
     */
    response(request: ARequest, response: AResponse, global?: any): Promise<AResponse>
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
    model?: typeof Model
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

export * from './middleware/types'
