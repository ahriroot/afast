import { Pool } from 'pg'
import DBPool from './db'
import { Model } from './model'

type JsonResponse = { [key: string]: any }

export type Config = {
    host?: string
    port?: number
    dev?: boolean
    dialect?: 'sqlite' | 'pg'
    database?: any
}

export class ARequest {
    method: string
    url: URL
    headers: { [x: string]: any }
    query: { [x: string]: any }
    params: { [x: string]: any }
    local: { [x: string]: any } = {}
    constructor(request: Request, params: { [x: string]: any } = {}) {
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

export type AResponse = JsonResponse | Response

export interface Middleware {
    request(request: ARequest): Promise<ARequest>
    response(request: ARequest, response: AResponse): Promise<AResponse>
}

export type Handler = (request: ARequest) => Promise<AResponse>

export interface View {
    model: Model
    allowed: string[]
}

class Router {
    router: {
        [string: string]: { handler: Handler; middlewares: Middleware[] }
    }
    views: { view: View; middlewares: Middleware[] } | undefined
    children: { [path: string]: Router }
    middlewares: Middleware[]
    constructor() {
        this.router = {}
        this.children = {}
        this.middlewares = []
    }

    /**
     * @param method
     * @param paths
     * @param handler
     * @param middlewares
     * @returns
     */
    method(
        method: string,
        paths: string[] | string,
        handler: Handler | View,
        middlewares: Middleware[] = [],
        end = false
    ) {
        if (typeof paths === 'string') {
            paths = paths.split('/').filter((x) => x !== '')
        }
        if (paths.length === 0) {
            const mi = [...this.middlewares, ...middlewares]
            if (method == 'VIEW') {
                this.views = {
                    view: handler as View,
                    middlewares: mi,
                }
                if (!end) {
                    if (!this.children[':primary']) {
                        this.children[':primary'] = new Router()
                    }
                    this.children[':primary'].method('VIEW', [], handler, mi, true)
                }
            } else {
                this.router[method] = {
                    handler: handler as Handler,
                    middlewares: mi,
                }
            }
        } else {
            const [path, ...rest] = paths
            if (!this.children[path]) {
                this.children[path] = new Router()
            }
            this.children[path].method(method, rest, handler, [...this.middlewares, ...middlewares])
        }
    }

    /**
     * @param paths
     * @param handler
     * @param middlewares
     * @returns
     * @description
     */
    get(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('GET', paths, handler, middlewares)
    }
    post(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('POST', paths, handler, middlewares)
    }
    put(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('PUT', paths, handler, middlewares)
    }
    patch(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('PATCH', paths, handler, middlewares)
    }
    delete(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('DELETE', paths, handler, middlewares)
    }
    view(paths: string[] | string, view: View, middlewares: Middleware[] = []) {
        return this.method('VIEW', paths, view, middlewares)
    }

    ws(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('GET', paths, handler, middlewares)
    }

    group(paths: string[] | string, middlewares: any[] = []): Router {
        if (typeof paths === 'string') {
            paths = paths.split('/').filter((x) => x !== '')
        }
        if (paths.length === 0) {
            this.middlewares = [...this.middlewares, ...middlewares]
            return this
        }
        const [path, ...rest] = paths
        if (!this.children[path]) {
            this.children[path] = new Router()
        }
        return this.children[path].group(rest, [...this.middlewares, ...middlewares])
    }

    async index(
        paths: string[],
        params: { [x: string]: any }
    ): Promise<{ router: Router; params: { [x: string]: any } } | undefined> {
        if (paths.length === 0) {
            return {
                router: this,
                params: params,
            }
        }
        const [path, ...rest] = paths
        if (!this.children[path]) {
            const key = Object.keys(this.children).find((x) => x[0] === ':')
            if (!key) {
                return undefined
            }
            const part = key.split(':')
            const p = { ...params }
            p[part[1]] = path
            if (part.length === 3) {
                switch (part[2]) {
                    case 'number':
                        p[part[1]] = Number(path)
                        break
                    case 'boolean':
                        p[part[1]] = path === 'false' ? false : Boolean(path)
                        break
                    default:
                        p[part[1]] = path
                        break
                }
            }
            return await this.children[key].index(rest, p)
        }
        return await this.children[path].index(rest, params)
    }
}

class App {
    root: Router
    constructor() {
        this.root = new Router()
    }

    print(router: Router, depth = 0): string {
        let result = ''
        for (const method in router.router) {
            result += `${'  '.repeat(depth)}${method}\n`
        }
        if (router.views) {
            result += `${'  '.repeat(depth)}VIEW\n`
        }
        for (const path in router.children) {
            result += `${'  '.repeat(depth)}${path}\n`
            result += this.print(router.children[path], depth + 1)
        }
        return result
    }

    map(): string {
        return this.print(this.root)
    }

    method(method: string, paths: string[] | string, handler: Handler | View, middlewares: Middleware[] = []) {
        this.root.method(method, paths, handler, middlewares)
    }
    get(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('GET', paths, handler, middlewares)
    }
    post(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('POST', paths, handler, middlewares)
    }
    put(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('PUT', paths, handler, middlewares)
    }
    patch(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('PATCH', paths, handler, middlewares)
    }
    delete(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('DELETE', paths, handler, middlewares)
    }
    view(paths: string[] | string, view: View, middlewares: Middleware[] = []) {
        return this.method('VIEW', paths, view, middlewares)
    }

    ws(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('GET', paths, handler, middlewares)
    }

    group(paths: string[] | string, middlewares: Middleware[] = []): Router {
        return this.root.group(paths, middlewares)
    }

    async index(paths: string[]): Promise<{ router: Router; params: { [x: string]: any } } | undefined> {
        return await this.root.index(paths, {})
    }

    run(
        config: Config = {
            port: 3000,
            host: '0.0.0.0',
            dev: false,
            dialect: 'sqlite',
        }
    ) {
        if (config.dialect && ['sqlite', 'pg'].includes(config.dialect)) {
            global.dialect = require(`./dialect/${config.dialect}`).default
            if (config.database) {
                if (config.dialect && ['sqlite', 'pg'].includes(config.dialect)) {
                    global.dialect = require(`./dialect/${config.dialect}`).default
                    if (config.database) {
                        global.pool = new DBPool(config.dialect, config.database)
                    }
                }
            }
        }
        const serve = Bun.serve({
            hostname: config.host,
            port: config.port,
            development: config.dev,
            fetch: async (request: Request, server) => {
                let req = new ARequest(request)
                const r = await this.index(req.url.pathname.split('/').filter((x) => x !== ''))
                if (!r) {
                    return new Response('Not Found', { status: 404 })
                }
                const { router, params } = r
                if (Object.keys(router.router).length === 0 && !router.views) {
                    return new Response('Not Found', { status: 404 })
                }
                req.setParams(params)
                const handler = router.router[request.method]

                let func: Handler
                let middlewares: Middleware[]

                if (!handler) {
                    if (!router.views) {
                        return new Response('Method Not Allowed', { status: 405 })
                    } else {
                        const views = router.views
                        const funcName = req.method.toLowerCase()
                        func = Object.getOwnPropertyDescriptor(views.view.constructor.prototype, funcName)?.value
                        if (!func) {
                            const allowed = views.view.allowed
                            if (allowed.includes(req.method)) {
                                const model = views.view.model
                                let json
                                switch (req.method) {
                                    case 'GET':
                                        if (req.params.primary) {
                                            json = await model.request_primary(req.params.primary)
                                        } else {
                                            json = await model.request_get(1, 10)
                                        }
                                        break
                                    case 'POST':
                                        json = await model.request_post(model)
                                        break
                                    case 'PUT':
                                        json = await model.request_put(1, {})
                                        break
                                    case 'PATCH':
                                        json = await model.request_put(1, {})
                                        break
                                    case 'DELETE':
                                        json = await model.request_delete(model)
                                        break
                                }
                                console.log(json)
                                return new Response(JSON.stringify({ sql: json }), {
                                    headers: {
                                        'Content-Type': 'application/json',
                                    },
                                })
                            }
                            return new Response('Method Not Allowed', { status: 405 })
                        }
                        middlewares = views.middlewares
                    }
                } else {
                    func = handler.handler
                    middlewares = handler.middlewares
                }

                if (
                    server.upgrade(request, {
                        data: {
                            unique: req.url.pathname,
                        },
                    })
                ) {
                    return
                }
                for (const middleware of middlewares) {
                    req = await middleware.request(req)
                }
                let resp = await func(req)
                for (let i = middlewares.length - 1; i >= 0; i--) {
                    const middleware = middlewares[i]
                    resp = await middleware.response(req, resp)
                }

                if (resp instanceof Response) {
                    return resp
                }
                return new Response(JSON.stringify(resp), {
                    headers: {
                        'Content-Type': 'application/json',
                    },
                })
            },
            websocket: {
                async open(ws) {
                    const data = ws.data as { unique: string }
                    console.log(`Connected: ${data.unique}`)
                },
                async message(ws, message) {
                    const data = ws.data as { unique: string }
                    console.log(`Received: ${message} from ${data.unique}`)
                },
                async close(ws) {
                    const data = ws.data as { unique: string }
                    console.log(`Disconnected: ${data.unique}`)
                },
            },
        })
        return serve
    }
}

const migrate = async (config: Config, models: (typeof Model)[], drop: boolean = false): Promise<void> => {
    if (config.dialect && ['sqlite', 'pg'].includes(config.dialect)) {
        global.dialect = require(`./dialect/${config.dialect}`).default
        if (config.database) {
            global.pool = new DBPool(config.dialect, config.database)
        }
    }

    models.forEach(async (model) => {
        const obj = new model()
        await obj.migrate(global.pool, drop)
    })
}

export default App
export { migrate }