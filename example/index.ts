import { App, Config, migrate, cors, ARequest } from 'afast'

import { world1 } from './handler/world1'
import { world2 } from './handler/world2'
import { world3 } from './handler/world3'
import { world4 } from './handler/world4'
import { UserView } from './handler/world5'
import { TestWebsocket } from './handler/world6'
import { M1 } from './middleware/m1'
import { M2 } from './middleware/m2'
import { MRes } from './middleware/mres'
import { Article } from './model/article'
import { User } from './model/user'

import cfg from './config.toml'

const app = new App()

app.use(
    cors({
        skip: (req, resp, global) => {
            return req.headers.host === '127.0.0.1:3000'
        },
    })
)

app.get('/', async (request: ARequest) => {
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
console.log(app.map())

const config = cfg as Config

config.global = {
    name: 'afast',
}

console.log('migrate start')
console.log(await migrate(config, [User, Article], false))
console.log('migrate end')

const server = app.run(config)

console.log(`Listening on ${server.url}`)
