import { ARequest, AResponse } from "afast"

export class MRes {
    async request(request: ARequest) {
        return request
    }

    async response(request: ARequest, response: AResponse) {
        return {
            code: 10000,
            msg: 'success',
            data: response,
        }
    }
}
