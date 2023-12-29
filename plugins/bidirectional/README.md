# Bidirectional Map

## Overview

The `Bidirectional` is a TypeScript class that provides a bidirectional mapping between keys and values. It allows you to associate keys with values and vice versa, and provides methods for setting, getting, and deleting these associations. Additionally, it supports automatic expiration of key-value pairs based on an optional expiration time.

## Usage

### Importing

```typescript
import { Bidirectional, newBidirectional } from 'afast/plugins/bidirectional'

// Create a new instance of Bidirectional
const bidirectionalMap: Bidirectional = new newBidirectional()

// Optionally, you can use the factory function with auto-clear feature
const autoClearBidirectionalMap: Bidirectional = newBidirectional(60) // Auto-clear expired entries every 60 seconds
```

### Setting Key-Value Pairs

```typescript
bidirectionalMap.setKV('key1', 'value1', 3600) // Expires in 1 hour
bidirectionalMap.setVK('value2', 'key2', 1800) // Expires in 30 minutes
```

### Getting Key or Value

```typescript
const key = bidirectionalMap.getKey('value1') // Output: 'key1'
const value = bidirectionalMap.getData('key2') // Output: 'value2'
```

### Deleting Key-Value Pair

```typescript
bidirectionalMap.delByKey('key1')
bidirectionalMap.delByData('value2')
```

### Retrieving All Keys or Values

```typescript
const allKeys = bidirectionalMap.allKey() // Output: { 'value2': 'key2' }
const allValues = bidirectionalMap.allData() // Output: { 'key2': 'value2' }
```

## Use in `afast`

```typescript
import { App, Config } from 'afast'
import { newBidirectional, Bidirectional } from 'afast/plugins/bidirectional'

const app = new App()

const storage = newBidirectional()

const global: { storage: Bidirectional } = {
    storage,
}

type Global = typeof global

app.get('/set/:key/:value', async (request, global: Global) => {
    global.storage.setKV(request.params.key, request.params.value)
    return {}
})

app.get('/key', async (request, global: Global) => {
    return {
        storage: await global.storage.allKey(),
    }
})

app.get('/data', async (request, global: Global) => {
    return {
        storage: await global.storage.allData(),
    }
})

app.get('/key/:value', async (request, global: Global) => {
    return {
        [request.params.value]: await global.storage.getKey(request.params.value),
    }
})

app.get('/value/:key', async (request, global: Global) => {
    return {
        [request.params.key]: await global.storage.getData(request.params.key),
    }
})

const config: Config = {
    global: global,
}

const server = app.run(config)

console.log(`Listening on ${server.url}`)
```
