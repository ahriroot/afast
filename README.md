# AFast Web Framework

[简体中文](./README.md) | [English](./README.en.md)

## 安装依赖

```bash
npm install -g bun
# npm install pg @types/pg  # 使用 postgres 时安装 
bun install
```

## 启动服务

```bash
bun run index.ts
```

## 示例

```bash
.
├── handler/
│   ├── world1.ts
│   ├── world2.ts
│   ├── world3.ts
│   ├── world4.ts
│   ├── world5.ts
│   └── world6.ts
├── middleware/
│   ├── m1.ts
│   ├── m2.ts
│   └── mres.ts
├── model/
│   ├── article.ts
│   └── user.ts
├── config.toml
└── index.ts
```

```toml
# config.toml
port = 3000
host = "localhost"
static = "./example/public"
dialect = "sqlite"

[database]
path = "./test.db"
```

```typescript
// handler/world1.ts
import { ARequest } from 'afast'

export const world1 = async (request: ARequest) => {
    return {
        hello: 'world1',
        headers: request.headers,
        params: request.params,
        query: request.query,
    }
}
```

```typescript
// handler/world2.ts
import { ARequest } from 'afast'

export const world2 = async (request: ARequest) => {
    return new Response('Hello, world2!')
}
```

```typescript
// handler/world3.ts
import { ARequest } from 'afast'

export const world3 = async (request: ARequest) => {
    return new Response('Hello, world3!')
}
```

```typescript
// handler/world4.ts
import { ARequest } from 'afast'

export const world4 = async (request: ARequest) => {
    const m1 = request.get('M1')
    const m2 = request.get('M2')
    console.log('Handler', m1, m2)
    return {
        hello: 'world3',
    }
}
```

```typescript
// handler/world5.ts
import { Model, View } from 'afast'
import { UserModel } from '../model/user'

export class UserView implements View {
    model: Model = new UserModel()
    allowed: string[] = ['GET', 'POST', 'PUT', 'DELETE']
}
```

```typescript
// handler/world6.ts
import { ARequest, Websocket, WsClient } from "afast"

export class TestWebsocket implements Websocket {
    clients: WsClient[]
    constructor() {
        this.clients = []
    }
    async open(ws: WsClient, request: ARequest) {
        ws.send('server connect')
        this.clients.push(ws)
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
```

```typescript
// middleware/m1.ts
import { ARequest, AResponse, Middleware, WsClient } from 'afast'

export class M1 implements Middleware {
    async request(request: ARequest, ws?: WsClient) {
        request.set('M1', 'M1 VAlue')
        console.log('M1 request')
        if (ws) {
            ws.send('M1 request ws')
        }
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M1', 'M1 VAlue')
        } else if (response instanceof Error) {
        } else {
            response['M1'] = 'M1 VAlue'
        }
        console.log('M1 response')
        return response
    }
}
```

```typescript
// middleware/m2.ts
import { ARequest, AResponse } from "afast"

export class M2 {
    async request(request: ARequest) {
        request.set('M2', 'M2 VAlue')
        console.log('M2 request')
        if (request.params['id'] === 1) {
            return new Response('M2 request return')
        }
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Response) {
            response.headers.set('M2', 'M2 VAlue')
        } else if (response instanceof Error) {
        } else {
            response['M2'] = 'M2 VAlue'
        }
        console.log('M2 response')
        return response
    }
}
```

```typescript
// middleware/mres.ts
import { ARequest, AResponse } from 'afast'

export class MRes {
    async request(request: ARequest) {
        return request
    }

    async response(request: ARequest, response: AResponse) {
        if (response instanceof Error) {
            return {
                code: 50000,
                msg: response.message,
                data: null,
            }
        }
        return {
            code: 10000,
            msg: 'success',
            data: response,
        }
    }
}
```

```typescript
// model/article.ts
import { Default, Model, fieldPrimary, fieldString, fieldTimestamp, fieldForeign } from 'afast'
import { UserModel } from './user'

export class ArticleModel extends Model {
    table() {
        return 'article'
    }
    id = fieldPrimary()
    title = fieldString()
    content = fieldString()
    created = fieldTimestamp({ default: Default.CURRENT_TIMESTAMP })
    user = fieldForeign({ foreign: UserModel, references: 'id' })
}
```

```typescript
// model/user.ts
import { Default, Model, fieldNumber, fieldPrimary, fieldString, fieldTimestamp } from 'afast'

export class UserModel extends Model {
    table() {
        return 'user'
    }
    id = fieldPrimary()
    name = fieldString()
    age = fieldNumber({ default: 0 })
    username = fieldString()
    password = fieldString({ show: false })
    created = fieldTimestamp({ default: Default.CURRENT_TIMESTAMP })
}
```

```typescript
// index.ts
import { App, Config, migrate } from 'afast'
import { world1 } from './handler/world1'
import { world2 } from './handler/world2'
import { world3 } from './handler/world3'
import { world4 } from './handler/world4'
import { UserView } from './handler/world5'
import { TestWebsocket } from './handler/world6'
import { M1 } from './middleware/m1'
import { M2 } from './middleware/m2'
import { MRes } from './middleware/mres'
import { ArticleModel } from './model/article'
import { UserModel } from './model/user'

import cfg from './config.toml'

const app = new App()
app.get('/', async (request) => {
    return {
        hello: 'world',
    }
})
app.get('/hello/world1/:id:number/:super:boolean/:name', world1, [new M2()])
app.get('/hello/world2', world2)
app.post('/hello/world3', world3)

const g = app.group('/hello', [new MRes()])
g.get('/world4', world4, [new M2()])
g.viewId('/world5', new UserView())
g.ws('/world6', new TestWebsocket(), [new M1()])

console.log(JSON.stringify(app.mapJson(), null, 4))

const config = cfg as Config

console.log('migrate start')
console.log(await migrate(config, [UserModel, ArticleModel], true))
console.log('migrate end')

const server = app.run(config)

console.log(`Listening on ${server.url}`)
```
