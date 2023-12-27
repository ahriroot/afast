import DBPool from './db'
import { ARequest } from './request'
import { Router } from './router'
import { AResponse, Config, Handler, Middleware, View, Websocket, WsClient } from './types'

export class App {
    root: Router
    wsClients: WeakMap<WsClient, Websocket>
    constructor() {
        this.root = new Router()
        this.wsClients = new WeakMap()
    }

    private print(router: Router, bg: boolean = true, depth = 0, paths: string[] = []): string {
        let result = ''
        for (const method in router.router) {
            const h = router.router[method].handler
            let handler
            if (h instanceof Function) {
                handler = h.name || '(anonymous)'
            } else {
                handler = h.constructor.name
            }
            if (bg) {
                handler = `\x1b[3m${handler}\x1b[0m`
            }
            let met = method
            if (bg) {
                switch (method) {
                    case 'GET':
                        met = '\x1b[32mGET\x1b[0m'
                        break
                    case 'POST':
                        met = '\x1b[33mPOST\x1b[0m'
                        break
                    case 'PUT':
                        met = '\x1b[35mPUT\x1b[0m'
                        break
                    case 'PATCH':
                        met = '\x1b[35mPUT\x1b[0m'
                        break
                    case 'DELETE':
                        met = '\x1b[31mDELETE\x1b[0m'
                        break
                    case 'WEBSOCKET':
                        met = '\x1b[36mDELETE\x1b[0m'
                        break
                    default:
                        met = '\x1b[34mDELETE\x1b[0m'
                        break
                }
            }
            result += `${'    '.repeat(depth)}${met} /${paths.join('/')} [${handler}]\n`
        }
        if (router.views) {
            let met = 'VIEW'
            if (bg) {
                met = '\x1b[34mVIEW\x1b[0m'
            }
            let handler = router.views.view.constructor.name
            if (bg) {
                handler = `\x1b[3m${handler}\x1b[0m`
            }
            result += `${'    '.repeat(depth)}${met} /${paths.join('/')} [${handler}]\n`
        }
        for (const path in router.children) {
            let p = path
            if (bg) {
                p = `\x1b[3m${path}\x1b[0m`
            }
            result += `${'    '.repeat(depth)}${p}\n`
            result += this.print(router.children[path], bg, depth + 1, [...paths, path])
        }
        return result
    }

    private printJson(router: Router, depth = 0, paths: string[] = []): any {
        let result: any = {}
        for (const method in router.router) {
            const h = router.router[method].handler
            let handler
            if (h instanceof Function) {
                handler = h.name || '(anonymous)'
            } else {
                handler = h.constructor.name
            }
            result[method] = {
                handler: handler,
                path: `/${paths.join('/')}`,
                middlewares: router.router[method].middlewares.map((x) => x.constructor.name),
            }
        }
        if (router.views) {
            result['VIEW'] = {
                handler: router.views.view.constructor.name,
                path: `/${paths.join('/')}`,
                middlewares: router.views.middlewares.map((x) => x.constructor.name),
            }
        }
        for (const path in router.children) {
            result[path] = this.printJson(router.children[path], depth + 1, [...paths, path])
        }
        return result
    }

    map(bg: boolean = true): string {
        return this.print(this.root, bg)
    }

    mapJson(): any {
        return this.printJson(this.root)
    }

    /**
     * Http request method
     * @param method <string> request method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler | view | websocket
     * @param middlewares <afast.Middleware[]> middleware
     */
    method(method: string, paths: string[] | string, handler: Handler | View, middlewares: Middleware[] = []) {
        this.root.method(method, paths, handler, middlewares)
    }

    /**
     * Http request get method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    get(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.root.get(paths, handler, middlewares)
    }

    /**
     * Http request post method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    post(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.root.post(paths, handler, middlewares)
    }

    /**
     * Http request put method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    put(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.root.put(paths, handler, middlewares)
    }

    /**
     * Http request patch method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    patch(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.root.patch(paths, handler, middlewares)
    }

    /**
     * Http request delete method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    delete(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.root.delete(paths, handler, middlewares)
    }

    /**
     * Http request view (auto generate CRUD)
     * @param paths <string | string[]> request path
     * @param view <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    view(paths: string[] | string, view: View, middlewares: Middleware[] = []) {
        return this.root.view(paths, view, middlewares)
    }

    /**
     * Http request view (auto generate CRUD), auto reqister `${path}/:primary` router
     * @param paths <string | string[]> request path
     * @param view <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    viewId(paths: string[] | string, view: View, middlewares: Middleware[] = []) {
        return this.root.viewId(paths, view, middlewares)
    }

    /**
     * Http request websocket
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    ws(paths: string[] | string, websocket: Websocket, middlewares: Middleware[] = []) {
        return this.root.ws(paths, websocket, middlewares)
    }

    /**
     * Create routing group
     * @param paths <string | string[]> request path
     * @param middlewares <afast.Middleware[]> middleware
     * @returns Router group
     */
    group(paths: string[] | string, middlewares: Middleware[] = []): Router {
        return this.root.group(paths, middlewares)
    }

    async index(paths: string[]): Promise<{ router: Router; params: { [x: string]: any } } | undefined> {
        return await this.root.index(paths, {})
    }

    /**
     * @param config <afast.Config { port?: number, host?: string, dev?: boolean, dialect?: 'sqlite' | 'pg' | 'mysql, database?: any}> config
     * @returns <Bun.Server> server
     * @description Run http server
     * ```typescript
     * const config: Config = {
     *    port: 3000,
     *    host: 'localhost',
     *    dialect: 'sqlite', // sqlite pg mysql
     *    database: {
     *        path: './test.db',
     *    },
     *    // database: {
     *        // host: '127.0.0.1',
     *        // port: 5432,
     *        // user: 'postgres',
     *        // pass: '',
     *        // name: 'afast',
     *    // },
     * }
     *
     * const server = app.run(config)
     * ```
     */
    run(
        config: Config = {
            port: 3000,
            host: '0.0.0.0',
            dev: false,
            static: './public',
            dialect: 'sqlite',
            view_allowed: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE'],
        }
    ) {
        const self = this
        if (config.dialect && ['sqlite', 'pg', 'mysql'].includes(config.dialect)) {
            global.dialect = require(`./dialect/${config.dialect}`).default
            if (config.database) {
                global.pool = new DBPool(config.dialect, config.database)
            }
        }
        if (config.static && config.static[config.static.length - 1] === '/') {
            config.static = config.static.slice(0, config.static.length - 1)
        }
        const serve = Bun.serve({
            hostname: config.host,
            port: config.port,
            development: config.dev,
            fetch: async (request: Request, server) => {
                // Parse request
                let req = await ARequest.parse(request)

                // Get router and parse params
                const r = await this.index(req.url.pathname.split('/').filter((x) => x !== ''))
                if (!r) {
                    // Try to get static file
                    if (config.static) {
                        let file = req.url.pathname
                        if (file === '/') {
                            file = '/index.html'
                        }
                        try {
                            const read = Bun.file(config.static + file)
                            if (!(await read.exists())) {
                                return new Response('Not Found', { status: 404 })
                            }
                            return new Response(await read.stream(), {
                                headers: {
                                    'Content-Type': read.type,
                                },
                            })
                        } catch (e: any) {
                            return new Response('Internal Server Error', { status: 500 })
                        }
                    }
                    return new Response('Not Found', { status: 404 })
                }

                // Get handler (router.router => {method: {handler, middlewares}})
                const { router, params } = r

                // Router not found
                if (Object.keys(router.router).length === 0 && !router.views) {
                    return new Response('Not Found', { status: 404 })
                }

                // Set params to req
                req.setParams(params)

                // Upgrade request to websocket
                if (
                    server.upgrade(request, {
                        data: {
                            req: req,
                            router: router,
                        },
                    })
                ) {
                    return
                }

                let func: Handler
                let middlewares: Middleware[]

                const handObject = router.router[request.method]

                if (!handObject) {
                    // Handler not found
                    if (!router.views) {
                        // View not found
                        return new Response('Not Found', { status: 404 })
                    } else {
                        const views = router.views

                        // Automatically call function by http method
                        const funcName = req.method.toLowerCase()

                        // Get function from view class
                        func = Object.getOwnPropertyDescriptor(views.view.constructor.prototype, funcName)?.value

                        // Get middlewares from view
                        middlewares = views.middlewares

                        // Call middlewares request
                        for (const middleware of middlewares) {
                            const r = await middleware.request(req)
                            if (r instanceof ARequest) {
                                req = r
                            } else {
                                if (r instanceof Response) {
                                    return r
                                } else if (r instanceof Error) {
                                    return new Response('Internal Server Error', { status: 500 })
                                }
                                return new Response(JSON.stringify(r || null), {
                                    headers: {
                                        'Content-Type': 'application/json',
                                    },
                                })
                            }
                        }

                        let resp: AResponse

                        try {
                            if (!func) {
                                // View function not found. Automatically call CRUD function

                                // View allowed http methods
                                let allowed = views.view.allowed
                                if (allowed === undefined) {
                                    allowed = config.view_allowed
                                }
                                if (allowed && !allowed.includes(req.method)) {
                                    return new Response('Method Not Allowed', { status: 405 })
                                }
                                // Get DB model from view
                                const model = views.view.model

                                if (!model) {
                                    return new Response('Not Found', { status: 404 })
                                }

                                switch (req.method) {
                                    case 'GET':
                                        if (req.params.primary === undefined) {
                                            resp = await model.request_get(
                                                req.query.page || 1,
                                                req.query.size || 10,
                                                req.query.sorts ? req.query.sorts.split(',') : []
                                            )
                                        } else {
                                            resp = await model.request_primary(req.params.primary)
                                        }
                                        break
                                    case 'POST':
                                        resp = await model.request_post(req.body)
                                        break
                                    case 'PUT':
                                        if (req.params.primary === undefined) {
                                            return new Response('Not Found', { status: 404 })
                                        }
                                        resp = await model.request_put(req.params.primary, req.body)
                                        break
                                    case 'PATCH':
                                        if (req.params.primary === undefined) {
                                            return new Response('Not Found', { status: 404 })
                                        }
                                        resp = await model.request_put(req.params.primary, req.body)
                                        break
                                    case 'DELETE':
                                        if (req.params.primary === undefined) {
                                            return new Response('Not Found', { status: 404 })
                                        }
                                        resp = await model.request_delete(req.params.primary)
                                        break
                                    default:
                                        return new Response('Method Not Allowed', { status: 405 })
                                }
                            } else {
                                // Call view function
                                resp = await func(req)
                            }
                        } catch (error: any) {
                            resp = error
                        }

                        // Call middlewares response
                        for (let i = middlewares.length - 1; i >= 0; i--) {
                            resp = await middlewares[i].response(req, resp || null)
                        }

                        if (resp instanceof Response) {
                            return resp
                        } else if (resp instanceof Error) {
                            return new Response('Internal Server Error', { status: 500 })
                        }
                        return new Response(JSON.stringify(resp || null), {
                            headers: {
                                'Content-Type': 'application/json',
                            },
                        })
                    }
                } else {
                    // Get handler and middlewares from router hand object
                    func = handObject.handler as Handler
                    middlewares = handObject.middlewares
                }

                // Call middlewares request
                for (const middleware of middlewares) {
                    const r = await middleware.request(req)
                    if (r instanceof ARequest) {
                        req = r
                    } else {
                        if (r instanceof Response) {
                            return r
                        } else if (r instanceof Error) {
                            return new Response('Internal Server Error', { status: 500 })
                        }
                        return new Response(JSON.stringify(r || null), {
                            headers: {
                                'Content-Type': 'application/json',
                            },
                        })
                    }
                }

                let resp
                try {
                    resp = await func(req)
                } catch (error: any) {
                    resp = error
                }

                if (resp === undefined) {
                    resp = null
                }

                // Call middlewares response
                for (let i = middlewares.length - 1; i >= 0; i--) {
                    const middleware = middlewares[i]
                    resp = await middleware.response(req, resp || null)
                }

                if (resp instanceof Response) {
                    return resp
                } else if (resp instanceof Error) {
                    return new Response('Internal Server Error', { status: 500 })
                }

                return new Response(JSON.stringify(resp || null), {
                    headers: {
                        'Content-Type': 'application/json',
                    },
                })
            },
            websocket: {
                async open(ws) {
                    const { req, router } = ws.data as {
                        req: ARequest
                        router: Router
                    }
                    const handler = router?.router['WEBSOCKET']?.handler as Websocket
                    const middlewares = router?.router['WEBSOCKET']?.middlewares as Middleware[]
                    if (!handler) {
                        ws.close()
                        return
                    }
                    let request = req
                    if (middlewares) {
                        for (const middleware of middlewares) {
                            const r = await middleware.request(req)
                            if (r instanceof ARequest) {
                                request = r
                            } else {
                                throw new Error('Middleware request error')
                            }
                        }
                    }
                    self.wsClients.set(ws, handler)
                    handler.open(ws, request)
                },
                async message(ws, message) {
                    const handler = self.wsClients.get(ws) as Websocket
                    if (message instanceof Buffer) {
                        message = Buffer.from(message).toString()
                    }
                    handler.message(ws, message)
                },
                async close(ws) {
                    const handler = self.wsClients.get(ws) as Websocket
                    handler.close(ws)
                },
            },
        })
        return serve
    }
}
