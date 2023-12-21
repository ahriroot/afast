import { BunFile } from "bun"

type JsonResponse = { [key: string]: any }

export type Config = {
    host?: string
    port?: number
    dev?: boolean
    dialect?: 'sqlite' | 'pg' | 'mysql'
    database?: any
}

export interface Middleware {
    request(request: ARequest): Promise<ARequest>
    response(request: ARequest, response: AResponse): Promise<AResponse>
}

export type Handler = (request: ARequest) => Promise<AResponse>

export interface View {
    model: Model
    allowed: string[]
}

export type AWebSocket = ServerWebSocket<unknown>

export interface Websocket {
    open(ws: AWebSocket, request: ARequest)
    message(ws: AWebSocket, message: string)
    close(ws: AWebSocket)
}

export type AResponse = JsonResponse | Response
