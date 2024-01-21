import { ARequest, AResponse } from "afast"

export class M2 {
    async request(request: ARequest) {
        request.set('M2', 'M2 Value')
        console.log('M2 request')
        if (request.params['id'] === 1) {
            return new Response('M2 request return')
        }
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M2', 'M2 Value')
        } else if (response instanceof Error) {
            response.message += ' M2 Value'
        } else if (response instanceof Object) {
            response['M2'] = 'M2 Value'
        } else {
            response = {
                'M2': 'M2 Value',
            }
        }
        console.log('M2 response')
        return response
    }
}
