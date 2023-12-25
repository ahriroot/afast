# AFast Web Framework

[简体中文](./README.md) | [English](./README.en.md)

## Tnstall dependencies

```bash
npm install -g bun
bun install
```

## Start server

```bash
bun run index.ts
```

## Example

```typescript
import { App, ARequest, AResponse, Config, Middleware, View, migrate } from 'afast'
import { Default, Model, fieldNumber, fieldPrimary, fieldString, fieldTimestamp } from 'afast'
import { WsClient, Websocket } from 'afast'

const app = new App()

// Create get request route
app.get('/hello/world1', async (request) => {
    return new Response('Hello, world2!')
})

// Create post request route
app.post('/hello/world3', async (request) => {
    return new Response('Hello, world3!')
})

// Create route with param
app.get('/hello/world2/:id:number/:super:boolean/:name', async (request) => {
    return {
        hello: 'world1',
        headers: request.headers,
        params: request.params,
        query: request.query,
    }
})

// Create Middleware
class M1 implements Middleware {
    // Only websocket request will pass ws
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

// Create Middleware
class M2 {
    async request(request: ARequest) {
        request.set('M2', 'M2 VAlue')
        console.log('M2 request')
        return request
    }

    async response(request: ARequest, response: AResponse) {
        // response type is Response or JsonObject
        if (response instanceof Response) {
            response.headers.set('M2', 'M2 VAlue')
        } else {
            response['M2'] = 'M2 VAlue'
        }
        console.log('M2 response')
        return response
    }
}

// Create Middleware to change response
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

// Create route group
const group = app.group('/hello', [new MiddlewareRes()])

// Create route based on route group
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

// Database model
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

// Create view (auto generate get post put delete route for crud)
class TestView implements View {
    model: Model = new TestModel()
    allowed: string[] = ['GET', 'POST', 'PUT', 'DELETE']
}

// Register view (viewId will auto register ${path}/:paimary)
// group.view('/world4', new TestView())
group.viewId('/world4', new TestView())

// Create websocket view
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

// Register websocket view
group.ws('/world5', new TestWebsocket(), [new M1()])

// Print route map
console.log(app.map())

// Config
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

// Migrate database
console.log('migrate start')
console.log(await migrate(config, [TestModel], true))  // true will drop table and recreate (use with caution)
console.log('migrate end')


// Start server
const server = app.run(config)

console.log(`Listening on ${server.url}`)
```
