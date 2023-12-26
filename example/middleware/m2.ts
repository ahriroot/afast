import { ARequest, AResponse } from "afast"

export class M2 {
    async request(request: ARequest) {
        request.set('M2', 'M2 VAlue')
        console.log('M2 request')
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M2', 'M2 VAlue')
        } else if (response instanceof Error) {
        } else {
            response['M2'] = 'M2 VAlue'
        }
        console.log('M2 response')
        return response
    }
}
