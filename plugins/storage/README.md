# Storage

## Overview

The `Storage` is a TypeScript class that implementation providing a simple key-value storage system with optional expiration of entries.

## Usage

### Importing

```typescript
import { Storage, newStorage } from 'afast/plugins/storage'

// Create a new instance of Storage
const storage: Storage = new newStorage()

// Use the factory function with auto-clear feature
const autoClearStorage: Storage = newStorage(60) // Auto-clear every 60 seconds
```

### Setting Key-Value Pairs

```typescript
storage.set('key1', { value: 'value1' }, 3600) // Expires in 1 hour
```

### Getting Value

```typescript
const value = storage.get('key1') // Output: 'value1'
```

### Deleting Key-Value Pair

```typescript
storage.del('key1')
```

### Retrieving All Key-Value Pairs

```typescript
const allEntries = storage.all()
console.log(allEntries) // Output: { 'key1': 'value1' }
```

## Use in `afast`

```typescript
import { App, Config } from 'afast'
import { newStorage, Storage } from 'afast/plugins/storage'

const app = new App()

const storage = newStorage()

const global: { storage: Storage } = {
    storage,
}

type Global = typeof global

app.get('/set/:key/:value', async (request, global: Global) => {
    global.storage.set(request.params.key, request.params.value, 3)
    return {}
})

app.get('/get', async (request, global: Global) => {
    return {
        storage: await global.storage.all(),
    }
})

app.get('/get/:key', async (request, global: Global) => {
    return {
        [request.params.key]: await global.storage.get(request.params.key),
    }
})

const config: Config = {
    global: global,
}

const server = app.run(config)

console.log(`Listening on ${server.url}`)
```
