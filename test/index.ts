import { App, ARequest, AResponse, Config, Middleware, View, migrate } from '../afast'
import { Default, Model, fieldNumber, fieldPrimary, fieldString, fieldTimestamp } from '../afast/model'
import { WsClient, Websocket } from '../afast/types'

const app = new App()

// 创建 get 请求的路由
app.get('/hello/world1', async (request) => {
    return new Response('Hello, world2!')
})

// 创建 post 请求的路由
app.post('/hello/world3', async (request) => {
    return new Response('Hello, world3!')
})

// 创建带 param 参数的路由
app.get('/hello/world2/:id:number/:super:boolean/:name', async (request) => {
    return {
        hello: 'world1',
        headers: request.headers,
        params: request.params,
        query: request.query,
    }
})

// 中间件
class M1 implements Middleware {
    // 只有 websocket 请求会传入 ws
    async request(request: ARequest, ws?: WsClient) {
        request.set('M1', 'M1 VAlue')
        console.log('M1 request')
        if (ws) {
            ws.send('M1 request ws')
            console.log('M1 request ws')
        }
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M1', 'M1 VAlue')
        } else {
            response['M1'] = 'M1 VAlue'
        }
        console.log('M1 response')
        return response
    }
}

// 中间件
class M2 {
    async request(request: ARequest) {
        request.set('M2', 'M2 VAlue')
        console.log('M2 request')
        return request
    }

    async response(request: ARequest, response: AResponse) {
        // response 类型可能是 Response 或 Json Object
        if (response instanceof Response) {
            response.headers.set('M2', 'M2 VAlue')
        } else {
            response['M2'] = 'M2 VAlue'
        }
        console.log('M2 response')
        return response
    }
}

// 包装 response 的中间件
class MiddlewareRes {
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

// 创建路由组
const group = app.group('/hello', [new MiddlewareRes()])

// 创建基于路由组的路由
group.get(
    '/world3',
    async (request) => {
        const m1 = request.get('M1')
        const m2 = request.get('M2')
        console.log('Handler', m1, m2)
        return {
            hello: 'world3',
        }
    },
    [new M2()]
)

// 数据库模型
class TestModel extends Model {
    table() {
        return 'test'
    }
    id = fieldPrimary()
    name = fieldString()
    age = fieldNumber({ default: 0 })
    username = fieldString()
    password = fieldString({ show: false })
    created = fieldTimestamp({ default: Default.CURRENT_TIMESTAMP })
}

// 视图 自动创建 get post put delete 用于 crud
class TestView implements View {
    model: Model = new TestModel()
    allowed: string[] = ['GET', 'POST', 'PUT', 'DELETE']
}

// 注册视图 viewId 会自动注册 ${path}/:paimary
// group.view('/world4', new TestView())
group.viewId('/world4', new TestView())

// websocket 视图
class TestWebsocket implements Websocket {
    async open(ws: WsClient, request: ARequest) {
        ws.send('server connect')
        console.log('connect', request.get('M1'))
    }
    async close(ws: WsClient) {
        ws.send('server disconnect')
        console.log('disconnect')
    }
    async message(ws: WsClient, msg: any) {
        ws.send('server message')
        console.log('message', msg)
    }
}

// 注册 websocket 视图
group.ws('/world5', new TestWebsocket(), [new M1()])

// 打印路由表
console.log(app.map())

// 配置
const config: Config = {
    port: 3000,
    host: 'localhost',
    dialect: 'sqlite',
    database: {
        path: './test.db',
    },
    // dialect: 'pg',
    // database: {
    //     host: '127.0.0.1',
    //     port: 5432,
    //     user: 'postgres',
    //     pass: 'Aa12345.',
    //     name: 'afast',
    // },
}

// 迁移数据库
console.log('migrate start')
console.log(await migrate(config, [TestModel], true))  // true 会删除表并重新创建 (谨慎使用)
console.log('migrate end')


// 启动服务
const server = app.run(config)

console.log(`Listening on ${server.url}`)
