# openapi-dry-validation-generator

🛠 This repository is WIP.
Generate dry-validation(ruby) from openapi spec file

## Summary

We want to output [dry-validation](https://github.com/dry-rb/dry-validation) from the OpenAPI definition file.
I want to be able to delegate the validation of the API to be implemented in Ruby to the OpenAPI definition 🚀.

The ultimate goal is to
- Provided as an executable file. (single binary, of course)
- Supports all definitions involving OpenAPI validation.
- Generate a ruby code within 1 second.

## Supports

This mark(🚫) is an option that we do not believe can be supported by `dry-validation` in the first place.

### Client Modification Feature

- [ ] 🚫 BasePath
- [ ] 🚫 Authorizations
- [ ] 🚫 UserAgent
- [ ] 🚫 MockServer

### Data Type Feature

- [ ] Custom
- [x] Int32
- [x] Int64
- [ ] Float
- [ ] Double
- [x] String
- [ ] Byte
- [x] Boolean
- [ ] Date
- [ ] DateTime
- [ ] Password
- [ ] Uuid
- [x] Array
- [ ] Null
- [ ] AnyType
- [x] Object
- [ ] Enum

### Documentation Feature

- [ ] Readme
- [ ] model
- [ ] Api

### Global Feature

- [ ] 🚫 Host
- [ ] 🚫 BasePath
- [ ] 🚫 Info
- [ ] Schemes
- [ ] PartialSchemes
- [ ] ExternalDocumentation
- [ ] Examples
- [ ] XMLStructureDefinitions
- [ ] 🚫 MultiServer
- [ ] ParameterizedServer
- [ ] ParameterStyling
- [ ] Callbacks
- [ ] LinkObjects

### Parameter Feature

- [ ] Path
- [x] Query
- [ ] Header
- [ ] Cookie

### Schema Support Feature

- [x] Simple
- [ ] Composite
- [ ] Polymorphism
- [ ] Union
- [ ] allOf
- [ ] anyOf
- [ ] oneOf
- [ ] not

### Security Feature

- [ ] 🚫 BasicAuth
- [ ] 🚫 ApiKey
- [ ] 🚫 OpenIDConnect
- [ ] 🚫 BearerToken
- [ ] 🚫 OAuth2_Implicit
- [ ] 🚫 OAuth2_Password
- [ ] 🚫 OAuth2_ClientCredentials
- [ ] 🚫 OAuth2_AuthorizationCode

### Input Format

- [x] JSON
- [x] YAML

## Contributors

- [ktanaka101](https://github.com/ktanaka101) - creator, maintainer

## License

MIT
