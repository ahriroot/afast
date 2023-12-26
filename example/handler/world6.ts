import { ARequest, Websocket, WsClient } from "afast"

export class TestWebsocket implements Websocket {
    async open(ws: WsClient, request: ARequest) {
        ws.send('server connect')
        console.log('connect', request.get('M1'))
    }
    async close(ws: WsClient) {
        ws.send('server disconnect')
        console.log('disconnect')
    }
    async message(ws: WsClient, msg: any) {
        ws.send('server message')
        console.log('message', msg)
    }
}
