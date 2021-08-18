# Ancon Protocol Rust Smart Contracts

## Documentation


### Handler

- `AddMetadata`
- `AddFile`

### Query

- `GetMetadata`
- `GetFile`
- `GetFileInfo`

### State
- Metadata
- MetadataSchema
- File

## Metadata JSON Schema

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

## Metadata Sample

```json
{
  "name": "XDV metadata sample: Press article",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "sources": ["QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"]
}
```

```json
{
  "name": "XDV metadata sample: Cientific Paper about Special Relativity Theory",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "refs": [
    "/ipfs/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "/ancon/z8mWaJHXieAVxxLagBpdaNWFEBKVWmMiE",
    "QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm"
  ],
}
```

```json
{
  "name": "XDV metadata sample: NFT",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "sources": ["QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"],
  "parent": "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "refs": [
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"
  ],
}
```
