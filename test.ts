import { ARequest, App } from '.'

const app = new App()

const router = app.descriptor()

app.get('/', async (request) => {
    return {
        hello: 'world',
    }
})

class Test {
    @router.get('/:p:number/:q:string')
    async index(request: ARequest) {
        console.log(request)
        return {
            hello: 'world',
        }
    }
}

const server = app.run()

console.log(`Listening on ${server.url}`)
