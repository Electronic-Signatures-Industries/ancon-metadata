
# 
# XDV Protocol Core API


<table>
  <tr>
   <td><strong>Author</strong>
   </td>
   <td>Rogelio Morrell Caballero, Stephanie Quezada
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

### Metadata


## `xdv.metadata.add(value, [options])`

> Adds a metadata based on XDV - 721.


### Parameters

| Name | Type | Description |
| ---- | ---- | ----------- |
| value | [CID][] | The content to publish |

### Options

> Pending

An optional object which may have the following keys:

| Name | Type | Default | Description |
| ---- | ---- | ------- | ----------- |
| resolve | `boolean` | `true` | Resolve given path before publishing |
| lifetime | `String` | `24h` | Time duration of the record |
| ttl | `String` | `undefined` | Time duration this record should be cached |
| key | `String` | `'self'` | Name of the key to be used |
| allowOffline | `boolean` | `true` | When offline, save the IPNS record to the the local datastore without broadcasting to the network instead of simply failing. |
| timeout | `Number` | `undefined` | A timeout in ms |
| signal | [AbortSignal][] | `undefined` |  Can be used to cancel any long running requests started as a result of this call |

### Returns

| Type | Description |
| -------- | -------- |
| `Promise<String>` | An object that contains the IPNS hash and the IPFS hash |

example of the returned object:

```JavaScript
{
  cid: "QmHash.."
}
```

### JSON Schema


```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://xdv.digital/v1/protocol/metadata",
  "title": "metadata",
  "description": "XDV Protocol metadata schema",
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
      };
      "links": {
          "type": "array",
          "description": "A set of ipld links",
      };
  };
  "required": [ "name", "description", "image", "links" ]
},
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


### TODO - Enroll

Business has a DB with vetted customers which are then used by the `XOPA-002` node.

### TODO - DID or Wallet whitelist

User binds the generated QR, which contains the public key for `Issuer` and enrolls wallet / DID address. `Issuer` signs a `XAdes` document
to keep it for auditing logs.


### TODO - Signing

Anytime the users is requested to sign, `XOPA-002` node will ask for:

- VC as VP to verify user with his issued credential
- If stale or revoked, needs to enroll again

### TODO - Verifying

#### Government

- Verifies `Issuer` certificate with PKI
- Verifies `XAdes` credential signed by `Issuer` which contains the key enrollment from the user.

#### Others

- Verifies `Verified Presentation (VP)` generated proof from `Holder` VC

##  Test Cases

### TODO

##  Copyright

Copyright Industrias de Firmas Electrónicas, S.A. (IFESA), 2021.