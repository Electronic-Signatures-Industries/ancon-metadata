
# 
# Ancon Protocol - Universal Metadata


<table>
  <tr>
   <td><strong>Author</strong>
   </td>
   <td>Rogelio Morrell Caballero
   </td>
  </tr>
  <tr>
   <td><strong>Category</strong>
   </td>
   <td>VC, DID, PKI, Cryptography, NFT
   </td>
  </tr>
  <tr>
   <td><strong>Created</strong>
   </td>
   <td>2021-08-11
   </td>
  </tr>
</table>



##   Simple Summary

Enables an IPLD protocol level Metadata and Files smart contract for use with data economy use cases.

##  Abstract

By having Metadata domain model be defined as a smart contract, we can accomplish more advanced scenarios when creating "data tokens", non fungible tokens NFT and securing offchain data computing.

IPLD schemas forms the base interface in XDV Protocol that makes in an agnostic way, worked with diverse linked data sets and at the same time, keep track or verified links.



## Specification

**Smart Contracts** in CosmWasm stores IPLD datasets in CosmWasm Storage as a Merkle Tree. Smart contracts are very light in features, and can only be called by **Ancon nodes** previously registered.

**Nodes** manages Ancon Smart Contracts, have a JSON-RPC and REST API interface, with a IPFS like gateway and REST and JSON-RPC API to add metadata or files. Nodes besides storing metadata onchain in Secret Network or any other pluggable ledger, can also support offchain storage in a pluggable merkle tree.

**Clients** in JavaScript and Go language have also a complete set of tooling, including SecretJS CosmJS client, XDV Universal Wallet, EthersJS and Veramo plus RSA forks off did-jwt and did-jwt-vc. **AnconJS** in Javascript will contains the primary implementation for Ancon Protocol.




### API

## `ancon.metadata.add(value, [options])`

> Adds an universal metadata.


### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| value | [Metadata][] | Metadata type |

> Metadata

| Name | Type | Description |
| ---- | ---- | ----------- |
| name | [string][] | Identifies the asset to which this metadata represents |
| description | [string][] | Describes the asset to which this token represents |
| image | [string][] | A URI pointing to a resource with mime type image/* representing the asset to which this token represents |
| sources | [array][] | Current intellectual property |
| owner | [string][] | The owner is a DID identifier |
| parent | [string][] | Transaction block |
| verifiedCredential | [string][] | Is the verified credential for the metadata |
| links | [string][] | References |

### Options

> Pending

An optional object which may have the following keys:

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| storage | `String` | `onchain` | Either `onchain` or `offchain` |


### Returns

| Type | Description |
| -------- | -------- |
| `Promise<AnconResponse>` | An object that contains the CID |

example of the returned object:

```JavaScript
{
  cid: "QmHash.."
}
```


## Metadata JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ancon.xdv.digital/v1/protocol/metadata",
  "title": "metadata",
  "description": "Ancon Protocol metadata schema",
  "type": "object",
  "properties": {
      "name": {
          "type": "string",
          "description": "Identifies the asset to which this token represents",
      },
      "description": {
          "type": "string",
          "description": "Describes the asset to which this token represents",
      },
      "image": {
          "type": "string",
          "description": "A URI pointing to a resource with mime type image/* representing the asset to which this token represents.",
      },
      "sources": {
          "type": "array",
          "description": "Current intellectual property",
      },
      "owner": {
          "type": "string",
          "description": "The owner is a DID identifier",
      },
      "parent": {
          "type": "string",
          "description": "Direct ascendant of the current intellectual property",
      },
      "verifiedCredential": {
          "type": "object",
          "description": "Is the verified credential for the metadata",
      },
      "links": {
          "type": "array",
          "description": "Sample of references included in the current intellectual property",
      }
  },
  "required": [ "name", "description", "image", "sources" ]
}
```

### Example

Imagine you want to publish your website under IPFS. You can use the [Files API](./FILES.md) to publish your static website and then you'll get a multihash you can link to. But when you need to make a change, a problem arises: you get a new multihash because you now have a different content. And it is not possible for you to be always giving others the new address.

Here's where the Name API comes in handy. With it, you can use one static multihash for your website under IPNS (InterPlanetary Name Service). This way, you can have one single multihash poiting to the newest version of your website.

```JavaScript

// TODO: Pending CosmJS integration

const payload = {
  "name": "XDV metadata sample",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "services": ["https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/"],
  "links": [
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "z8mWaJHXieAVxxLagBpdaNWFEBKVWmMiE",
    "QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm",
  ],
};


const res = await xdv.metadata.add(payload)
console.log(`https://gateway.xdv.digital/xdv/${res.cid}`)
```

This way, you can republish a new version of your website under the same address. By default, `ipfs.name.publish` will use the Peer ID. If you want to have multiple websites (for example) under the same IPFS module, you can always check the [key API](./KEY.md).

A great source of [examples][] can be found in the tests for this API.

### Notes

The `allowOffline` option is not yet implemented in js-ipfs. See tracking issue [ipfs/js-ipfs#1997](https://github.com/ipfs/js-ipfs/issues/1997).

## `ancon.files.add(value, [options])`

> Adds a file.


### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| value | [Metadata][] | Metadata type |

> Metadata

| Name | Type | Description |
| ---- | ---- | ----------- |
| name | [string][] | Identifies the asset to which this metadata represents |
| description | [string][] | Describes the asset to which this token represents |
| image | [string][] | A URI pointing to a resource with mime type image/* representing the asset to which this token represents |
| sources | [array][] | Current intellectual property |
| owner | [string][] | The owner is a DID identifier |
| parent | [string][] | Transaction block |
| verifiedCredential | [string][] | Is the verified credential for the metadata |
| links | [string][] | References |

### Options

> Pending

An optional object which may have the following keys:

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| storage | `String` | `onchain` | Either `onchain` or `offchain` |


### Returns

| Type | Description |
| -------- | -------- |
| `Promise<AnconResponse>` | An object that contains the CID |

example of the returned object:

```JavaScript
{
  cid: "QmHash.."
}
```


## Metadata JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ancon.xdv.digital/v1/protocol/metadata",
  "title": "metadata",
  "description": "Ancon Protocol metadata schema",
  "type": "object",
  "properties": {
      "name": {
          "type": "string",
          "description": "Identifies the asset to which this token represents",
      },
      "description": {
          "type": "string",
          "description": "Describes the asset to which this token represents",
      },
      "image": {
          "type": "string",
          "description": "A URI pointing to a resource with mime type image/* representing the asset to which this token represents.",
      },
      "sources": {
          "type": "array",
          "description": "Current intellectual property",
      },
      "owner": {
          "type": "string",
          "description": "The owner is a DID identifier",
      },
      "parent": {
          "type": "string",
          "description": "Direct ascendant of the current intellectual property",
      },
      "verifiedCredential": {
          "type": "object",
          "description": "Is the verified credential for the metadata",
      },
      "links": {
          "type": "array",
          "description": "Sample of references included in the current intellectual property",
      }
  },
  "required": [ "name", "description", "image", "sources" ]
}
```

### Example

Imagine you want to publish your website under IPFS. You can use the [Files API](./FILES.md) to publish your static website and then you'll get a multihash you can link to. But when you need to make a change, a problem arises: you get a new multihash because you now have a different content. And it is not possible for you to be always giving others the new address.

Here's where the Name API comes in handy. With it, you can use one static multihash for your website under IPNS (InterPlanetary Name Service). This way, you can have one single multihash poiting to the newest version of your website.

```JavaScript

// TODO: Pending CosmJS integration

const payload = {
  "name": "XDV metadata sample",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "services": ["https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/"],
  "links": [
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "z8mWaJHXieAVxxLagBpdaNWFEBKVWmMiE",
    "QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm",
  ],
};


const res = await xdv.metadata.add(payload)
console.log(`https://gateway.xdv.digital/xdv/${res.cid}`)
```

This way, you can republish a new version of your website under the same address. By default, `ipfs.name.publish` will use the Peer ID. If you want to have multiple websites (for example) under the same IPFS module, you can always check the [key API](./KEY.md).

A great source of [examples][] can be found in the tests for this API.

### Notes

The `allowOffline` option is not yet implemented in js-ipfs. See tracking issue [ipfs/js-ipfs#1997](https://github.com/ipfs/js-ipfs/issues/1997).


## `ancon.metadata.get(value, [options])`

> Adds an universal metadata.


### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| value | [Metadata][] | Metadata type |

> Metadata

| Name | Type | Description |
| ---- | ---- | ----------- |
| name | [string][] | Identifies the asset to which this metadata represents |
| description | [string][] | Describes the asset to which this token represents |
| image | [string][] | A URI pointing to a resource with mime type image/* representing the asset to which this token represents |
| sources | [array][] | Current intellectual property |
| owner | [string][] | The owner is a DID identifier |
| parent | [string][] | Transaction block |
| verifiedCredential | [string][] | Is the verified credential for the metadata |
| links | [string][] | References |

### Options

> Pending

An optional object which may have the following keys:

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| storage | `String` | `onchain` | Either `onchain` or `offchain` |


### Returns

| Type | Description |
| -------- | -------- |
| `Promise<AnconResponse>` | An object that contains the CID |

example of the returned object:

```JavaScript
{
  cid: "QmHash.."
}
```


## Metadata JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ancon.xdv.digital/v1/protocol/metadata",
  "title": "metadata",
  "description": "Ancon Protocol metadata schema",
  "type": "object",
  "properties": {
      "name": {
          "type": "string",
          "description": "Identifies the asset to which this token represents",
      },
      "description": {
          "type": "string",
          "description": "Describes the asset to which this token represents",
      },
      "image": {
          "type": "string",
          "description": "A URI pointing to a resource with mime type image/* representing the asset to which this token represents.",
      },
      "sources": {
          "type": "array",
          "description": "Current intellectual property",
      },
      "owner": {
          "type": "string",
          "description": "The owner is a DID identifier",
      },
      "parent": {
          "type": "string",
          "description": "Direct ascendant of the current intellectual property",
      },
      "verifiedCredential": {
          "type": "object",
          "description": "Is the verified credential for the metadata",
      },
      "links": {
          "type": "array",
          "description": "Sample of references included in the current intellectual property",
      }
  },
  "required": [ "name", "description", "image", "sources" ]
}
```

### Example

Imagine you want to publish your website under IPFS. You can use the [Files API](./FILES.md) to publish your static website and then you'll get a multihash you can link to. But when you need to make a change, a problem arises: you get a new multihash because you now have a different content. And it is not possible for you to be always giving others the new address.

Here's where the Name API comes in handy. With it, you can use one static multihash for your website under IPNS (InterPlanetary Name Service). This way, you can have one single multihash poiting to the newest version of your website.

```JavaScript

// TODO: Pending CosmJS integration

const payload = {
  "name": "XDV metadata sample",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "services": ["https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/"],
  "links": [
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "z8mWaJHXieAVxxLagBpdaNWFEBKVWmMiE",
    "QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm",
  ],
};


const res = await xdv.metadata.add(payload)
console.log(`https://gateway.xdv.digital/xdv/${res.cid}`)
```

This way, you can republish a new version of your website under the same address. By default, `ipfs.name.publish` will use the Peer ID. If you want to have multiple websites (for example) under the same IPFS module, you can always check the [key API](./KEY.md).

A great source of [examples][] can be found in the tests for this API.

### Notes

The `allowOffline` option is not yet implemented in js-ipfs. See tracking issue [ipfs/js-ipfs#1997](https://github.com/ipfs/js-ipfs/issues/1997).



## `ancon.files.get(value, [options])`

> Adds an universal metadata.


### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| value | [Metadata][] | Metadata type |

> Metadata

| Name | Type | Description |
| ---- | ---- | ----------- |
| name | [string][] | Identifies the asset to which this metadata represents |
| description | [string][] | Describes the asset to which this token represents |
| image | [string][] | A URI pointing to a resource with mime type image/* representing the asset to which this token represents |
| sources | [array][] | Current intellectual property |
| owner | [string][] | The owner is a DID identifier |
| parent | [string][] | Transaction block |
| verifiedCredential | [string][] | Is the verified credential for the metadata |
| links | [string][] | References |

### Options

> Pending

An optional object which may have the following keys:

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| storage | `String` | `onchain` | Either `onchain` or `offchain` |


### Returns

| Type | Description |
| -------- | -------- |
| `Promise<AnconResponse>` | An object that contains the CID |

example of the returned object:

```JavaScript
{
  cid: "QmHash.."
}
```


## Metadata JSON Schema

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://ancon.xdv.digital/v1/protocol/metadata",
  "title": "metadata",
  "description": "Ancon Protocol metadata schema",
  "type": "object",
  "properties": {
      "name": {
          "type": "string",
          "description": "Identifies the asset to which this token represents",
      },
      "description": {
          "type": "string",
          "description": "Describes the asset to which this token represents",
      },
      "image": {
          "type": "string",
          "description": "A URI pointing to a resource with mime type image/* representing the asset to which this token represents.",
      },
      "sources": {
          "type": "array",
          "description": "Current intellectual property",
      },
      "owner": {
          "type": "string",
          "description": "The owner is a DID identifier",
      },
      "parent": {
          "type": "string",
          "description": "Direct ascendant of the current intellectual property",
      },
      "verifiedCredential": {
          "type": "object",
          "description": "Is the verified credential for the metadata",
      },
      "links": {
          "type": "array",
          "description": "Sample of references included in the current intellectual property",
      }
  },
  "required": [ "name", "description", "image", "sources" ]
}
```

### Example

Imagine you want to publish your website under IPFS. You can use the [Files API](./FILES.md) to publish your static website and then you'll get a multihash you can link to. But when you need to make a change, a problem arises: you get a new multihash because you now have a different content. And it is not possible for you to be always giving others the new address.

Here's where the Name API comes in handy. With it, you can use one static multihash for your website under IPNS (InterPlanetary Name Service). This way, you can have one single multihash poiting to the newest version of your website.

```JavaScript

// TODO: Pending CosmJS integration

const payload = {
  "name": "XDV metadata sample",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "services": ["https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/",
  "https://explore.ipld.io/#/explore/"],
  "links": [
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "z8mWaJHXieAVxxLagBpdaNWFEBKVWmMiE",
    "QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm",
  ],
};


const res = await xdv.metadata.add(payload)
console.log(`https://gateway.xdv.digital/xdv/${res.cid}`)
```

This way, you can republish a new version of your website under the same address. By default, `ipfs.name.publish` will use the Peer ID. If you want to have multiple websites (for example) under the same IPFS module, you can always check the [key API](./KEY.md).

A great source of [examples][] can be found in the tests for this API.

### Notes

The `allowOffline` option is not yet implemented in js-ipfs. See tracking issue [ipfs/js-ipfs#1997](https://github.com/ipfs/js-ipfs/issues/1997).


##  Copyright

Copyright Industrias de Firmas Electrónicas, S.A. (IFESA), 2021.