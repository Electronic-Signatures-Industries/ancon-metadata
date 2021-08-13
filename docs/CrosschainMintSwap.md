
# 
# XDV Protocol Crosschain Mint/Swap API


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

Implement a DID for documents or identities that can be legally verifiable using qualified signatures from smart card or HSM.

##  Abstract

We propose a novel way to  use old technology with newer technology like Verified Credentials (VC) model. In `did:xdv`, an `Issuer` has a government mandated
smart card containing RSA keys compatible with either PKCS#11 or PKCS#12. `Issuer` can sign VC models and also XML with `XAdes` which stands for `XML Advanced Electronic Signatures`.

These combination of signatures can be further extended with `IPLD` as linked data schema. The `Holder` then can use this credential to interact with the `Issuer`
or `Verifier`.


##  Motivation

In previous hackathon organized by Escala Latam, BID / IDB Labs and `Camara Panamena de Tecnologia (CAPATEC)`, there was a challenge that has never been able to be resolved
by the customer, **How can I extend the trust from my smart cards keys to allow a set of "verified/trusted" users sign documents with other keys and still be trustable**


##  Prior Art

We already explored in `XOPA-001` a way to key exchange or VC exchange using a data model based on a scanned identity card. But is very narrow target, so we went
back to the drawing board and research the existing digital signatures laws in Panama.

We found not only `PAdes` is legal, but also `XAdes` and `CAdes`. With that in place, and with our fork off `did-jwt` called `did-jwt-rsa`, we fork the next library
for this use case `did-jwt-vc` to `did-jwt-rsa-vc`.

Thus the specification contains the following prior arts and implementations:

- **did-jwt** and **did-jwt-vc** forked libraries for RSA cryptosuite
- XDV Signer with XAdes signing enabled
- XDV Protocol (under development)
- XDV Universal Wallet



## Specification

### TODO - Offchain KYC

This KYC is done before any `XOPA-002` application can start enrolling users.

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