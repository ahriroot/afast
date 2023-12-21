import { Handler, Middleware, View, Websocket } from './types'

/**
 * @description Router
 */
export class Router {
    router: {
        [string: string]: { handler: Handler | Websocket; middlewares: Middleware[] }
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
     * Http request method
     * @param method <string> request method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler | view | websocket
     * @param middlewares <afast.Middleware[]> middleware
     */
    method(
        method: string,
        paths: string[] | string,
        handler: Handler | View | Websocket,
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
            this.children[path].method(method, rest, handler, [...this.middlewares, ...middlewares], end)
        }
    }

    /**
     * Http request get method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    get(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('GET', paths, handler, middlewares)
    }

    /**
     * Http request post method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    post(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('POST', paths, handler, middlewares)
    }

    /**
     * Http request put method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    put(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('PUT', paths, handler, middlewares)
    }

    /**
     * Http request patch method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    patch(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('PATCH', paths, handler, middlewares)
    }

    /**
     * Http request delete method
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing handler
     * @param middlewares <afast.Middleware[]> middleware
     */
    delete(paths: string[] | string, handler: Handler, middlewares: Middleware[] = []) {
        return this.method('DELETE', paths, handler, middlewares)
    }

    /**
     * Http request view (auto generate CRUD)
     * @param paths <string | string[]> request path
     * @param view <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    view(paths: string[] | string, view: View, middlewares: Middleware[] = []) {
        return this.method('VIEW', paths, view, middlewares, true)
    }

    /**
     * Http request view (auto generate CRUD), auto reqister `${path}/:primary` router
     * @param paths <string | string[]> request path
     * @param view <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    viewId(paths: string[] | string, view: View, middlewares: Middleware[] = []) {
        return this.method('VIEW', paths, view, middlewares, false)
    }

    /**
     * Http request websocket
     * @param paths <string | string[]> request path
     * @param handler <afast.Handler> routing view
     * @param middlewares <afast.Middleware[]> middleware
     */
    ws(paths: string[] | string, websocket: Websocket, middlewares: Middleware[] = []) {
        return this.method('GET', paths, websocket, middlewares)
    }

    /**
     * Create routing group
     * @param paths <string | string[]> request path
     * @param middlewares <afast.Middleware[]> middleware
     * @returns Router group
     */
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

    /**
     * Index the handler by path
     * @param paths <string[]> request path
     * @param params <{ [x: string]: any }> request params
     * @returns <afast.Router> router include handler and middlewares
     */
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
