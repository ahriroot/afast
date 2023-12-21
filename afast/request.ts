export class ARequest {
    request: Request
    method: string
    url: URL
    headers: { [x: string]: any }
    query: { [x: string]: any }
    params: { [x: string]: any }
    local: { [x: string]: any } = {}
    body: any
    constructor(request: Request, params: { [x: string]: any } = {}) {
        this.request = request
        this.url = new URL(request.url)
        this.method = request.method
        let headers: { [x: string]: any } = {}
        for (const [key, value] of request.headers) {
            headers[key] = value
        }
        this.headers = headers
        let query: { [x: string]: any } = {}
        for (const [key, value] of this.url.searchParams) {
            query[key] = value
        }
        this.query = query
        this.params = params
    }

    static async parse(request: Request, params: { [x: string]: any } = {}) {
        const req = new ARequest(request, params)
        await req.parseBody()
        return req
    }

    async parseBody() {
        const contentType = this.headers['content-type']
        if (contentType && contentType.includes('application/json') && this.request.body) {
            this.body = await this.request.json()
        } else {
            this.body = await this.request.text()
        }
    }

    setParams(params: { [x: string]: any }) {
        this.params = params
    }

    set(key: string, value: any) {
        this.local[key] = value
    }

    get(key: string) {
        return this.local[key]
    }
}
