# XDV Protocol Rust Smart Contracts

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
      "parent": {
          "type": "array",
          "description": "Direct ascendant of the current intellectual property",
      },
      "references": {
          "type": "array",
          "description": "Sample of references included in the current intellectual property",
      },
      "kind": {
        "type": "string",
        "description": "Type of linked metadata compatible with schema.org",
      }
  },
  "required": [ "name", "description", "image", "sources" ]
},
```

## Metadata Sample

```json
{
  "name": "XDV metadata sample: Press article",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "sources": ["QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"],
  "kind": "Publication"
}
```

```json
{
  "name": "XDV metadata sample: Cientific Paper about Special Relativity Theory",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "links": [
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "z8mWaJHXieAVxxLagBpdaNWFEBKVWmMiE",
    "QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm"
  ],
  "service": [
    "https://explore.ipld.io/#/explore/",
    "https://explore.ipld.io/#/explore/",
    "https://explore.ipld.io/#/explore/"
  ],
  "kind": "Publication"
}
```

```json
{
  "name": "XDV metadata sample: NFT",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "sources": ["QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"],
  "parent": ["QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"],
  "references": [
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D"
  ],
  "kind": "NFT"
}
```
