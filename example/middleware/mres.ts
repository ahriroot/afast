import { ARequest, AResponse } from 'afast'

export class MRes {
    async request(request: ARequest) {
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Error) {
            return {
                code: 50000,
                msg: response.message,
                data: null,
            }
        }
        return {
            code: 10000,
            msg: 'success',
            data: response,
        }
    }
}
