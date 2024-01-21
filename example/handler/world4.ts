import { ARequest } from "afast"

export const world4 = async (request: ARequest) => {
    const m1 = request.get('M1')
    const m2 = request.get('M2')
    console.log('Handler', m1, m2)
    return {
        hello: 'world4',
    }
}
