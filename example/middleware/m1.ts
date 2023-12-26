import { ARequest, AResponse, Middleware, WsClient } from "afast"

export class M1 implements Middleware {
    async request(request: ARequest, ws?: WsClient) {
        request.set('M1', 'M1 VAlue')
        console.log('M1 request')
        if (ws) {
            ws.send('M1 request')
        }
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M1', 'M1 VAlue')
        } else {
            response['M1'] = 'M1 VAlue'
        }
        console.log('M1 response')
        return response
    }
}
