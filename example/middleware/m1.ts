import { ARequest, AResponse, Middleware, WsClient } from 'afast'

export class M1 implements Middleware {
    async request(request: ARequest, ws?: WsClient) {
        request.set('M1', 'M1 Value')
        console.log('M1 request')
        if (ws) {
            ws.send('M1 request ws')
        }
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M1', 'M1 Value')
        } else if (response instanceof Error) {
            response.message += ' M1 Value'
        } else {
            response['M1'] = 'M1 Value'
        }
        console.log('M1 response')
        return response
    }
}
