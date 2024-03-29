import { ARequest, Websocket, WsClient } from "afast"

export class TestWebsocket implements Websocket {
    clients: WsClient[]
    constructor() {
        this.clients = []
    }
    async open(ws: WsClient, request: ARequest) {
        ws.send('server connect')
        this.clients.push(ws)
        console.log('connect', request.get('M1'))
    }
    async close(ws: WsClient) {
        ws.send('server disconnect')
        this.clients.splice(this.clients.indexOf(ws), 1)
        console.log('disconnect')
    }
    async message(ws: WsClient, msg: any) {
        ws.send('server message')
        console.log('message', msg)
    }
}
