import { ARequest, App } from '.'

const app = new App()

const router = app.descriptor()

app.get('/', async (request) => {
    return {
        hello: 'world',
    }
})

const g = router.group('/g')

class Test {
    @g.get('/:p:number/:q:string')
    async index(request: ARequest) {
        return request
    }
}

const server = app.run()

console.log(`Listening on ${server.url}`)
