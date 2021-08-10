# XDV Rust Protocol Smart Contracts

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
      };
      "links": {
          "type": "array",
          "description": "A set of ipld links",
      };
  };
  "required": [ "name", "description", "image" ]
},
```

## Metadata Sample

```json
{
  "name": "XDV metadata sample",
  "description": "testing sample",
  "image": "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
  "links": [
    "https://explore.ipld.io/#/explore/QmSnuWmxptJZdLJpKRarxBMS2Ju2oANVrgbr2xWbie9b2D",
    "https://explore.ipld.io/#/explore/z8mWaJHXieAVxxLagBpdaNWFEBKVWmMiE",
    "https://explore.ipld.io/#/explore/QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm",
  ],
}
```
