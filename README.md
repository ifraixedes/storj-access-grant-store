# Access Grant Store

Access grant store is library for storing
[Storj Access Grants](https://docs.storj.io/dcs/concepts/access/access-grants/)
in a text file with some specific format, offering the following capabilities:

- Store user's metadata associated to them for facilitating the management.
- Add / Read / Update / Delete  an access grant.
- Search them by tags and/or fields defined by the user.

The access grant is encrypted before storing it in the file.

## Roadmap

NOTE that there isn't any commitment on this roadmap.

### v1

- [X] Define text file format. Extension is `.ags` (Access Grant Store).
- [ ] Implement parser.
- [ ] Implement API for reading the file.
- [ ] Implement API for writing the file.
- [ ] Define encryption for access grants values based on password.
- [ ] Implement API for operations:
  - [ ] Add an access grant.
  - [ ] Get an access grant.
  - [ ] Update an access grant.
  - [ ] Delete an access grant.
  - [ ] List the access grants applying an optional filter by tags and/or user defined fields.

### v2

These is not a complete list, they are mostly ideas that I think that could be good for the second
release or a further release.

- [ ] Support encryption with other mechanism (GPG, SSH keys).
- [ ] Support encryption through key rings. This needs investigation if it's possible before
  implementing anything.
