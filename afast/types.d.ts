import { Model } from './model'

type JsonResponse = { [key: string]: any }

export type Config = {
    host?: string
    port?: number
    dev?: boolean
    dialect?: 'sqlite' | 'pg' | 'mysql'
    database?: any
}

export type WsClient = ServerWebSocket<unknown>

export interface Middleware {
    /**
     * @param request <afast.ARequest> request
     * @param ws <afast.WsClient> websocket client (only for websocket)
     * @returns <afast.ARequest> request
     */
    request(request: ARequest, ws?: WsClient): Promise<ARequest>
    response(request: ARequest, response: AResponse): Promise<AResponse>
}

export type Handler = (request: ARequest) => Promise<AResponse>

export interface View {
    model?: Model
    allowed?: string[]
}

export interface Websocket {
    open(ws: AWebSocket, request: ARequest)
    message(ws: AWebSocket, message: string)
    close(ws: AWebSocket)
}

export type AResponse = JsonResponse | Response
