import App, { ARequest, AResponse, Config, Middleware, View, migrate } from './afast'
import {} from './afast'
import { Model, fieldNumber, fieldPrimary, fieldString, Default, fieldTimestamp } from './afast/model'

const app = new App()

app.get('/hello/world1/:id:number/:super:boolean/:name', async (request) => {
    return {
        hello: 'world1',
        headers: request.headers,
        params: request.params,
        query: request.query,
    }
})
app.get('/hello/world2', async (request) => {
    return new Response('Hello, world2!')
})
app.post('/hello/world3', async (request) => {
    return new Response('Hello, world3!')
})

class M1 implements Middleware {
    async request(request: ARequest) {
        request.set('M1', 'M1 VAlue')
        console.log('M1 request')
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

class M2 {
    async request(request: ARequest) {
        request.set('M2', 'M2 VAlue')
        console.log('M2 request')
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M2', 'M2 VAlue')
        } else {
            response['M2'] = 'M2 VAlue'
        }
        console.log('M2 response')
        return response
    }
}

const g = app.group('/hello', [new M1()])
g.get(
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

const def = () => {
    return new Date().getTime()
}

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

class TestView implements View {
    model: Model
    allowed: string[]
    constructor() {
        this.model = new TestModel()
        this.allowed = ['GET', 'POST']
    }
}

const v = new TestView()

console.log(await v.model.request_primary(1))
console.log(await v.model.request_get(1, 10))
console.log(await v.model.request_get(1, 10, ['id', '-name']))
console.log(await v.model.request_post({ name: 'ahri', age: 12, username: 'ahriknow', password: 'password' }))
console.log(await v.model.request_put(1, { name: 'ahri', age: 12, username: 'ahriknow', password: 'password' }))
console.log(await v.model.request_delete(1))

g.view('/world4', v, [new M2()])

console.log(app.map())

const config: Config = {
    port: 3000,
    host: 'localhost',
    // dialect: 'sqlite',
    // database: {
    //     path: './test.db',
    // },
    dialect: 'pg',
    database: {
        host: '127.0.0.1',
        port: 5432,
        user: 'postgres',
        pass: 'Aa12345.',
        name: 'afast',
    },
}

console.log('migrate start')
await migrate(config, [TestModel], true)
console.log('migrate end')

// const server = app.run(config)

// console.log(`Listening on ${server.url}`)
