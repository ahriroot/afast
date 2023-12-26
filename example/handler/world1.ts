import { ARequest } from "afast"

export const world1 = async (request: ARequest) => {
    return {
        hello: 'world1',
        headers: request.headers,
        params: request.params,
        query: request.query,
    }
}
