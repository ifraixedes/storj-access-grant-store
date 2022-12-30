# Text File Format

The file format is very strict, it doesn't only use specific characters for defining fields and
associated values, it also impose rules:
- Fields has to be in a specific order.
- All the values of non-composed fields are strings.
- The values of some fields have a set of accepted characters and specific delimiters.
- The indentation is enforced; it uses tabs and MUST appear with a specific number in determined
  cases.
- Number of blank lines between sections and fields and values of the fields that they are lists is
  fixed.

The file uses the `.ags` extension.

## Specification

Let's see an example:

```
# project-name

## restricted-access-grant-name
grant = this is the access grant encrypted
tags = one, two, three
description = this is a single line string as a TOML one but withotu requiring quotes
notes =
This is a multiline string as a TOML one but it doesn't require quotes.
The last empty line before "permissions" field is not considered part of the notes.

permissions =
- bucket-name-1
	prefix1/: read, write
	prefix2/:
	prefix3/: write, read, delete, list
- bucket-name-2
	/:

metadata =
- field1: this is the value
- field 2: Spaces in the field name are valid
- user\:10: Colons are allowed in the field's name but they have to be scaped with back slash
- field 3:
	Multiple lines are OK. The text has to start in the line after the field name and
	each new line has to be indended by one tab, which are not considered part of the
	text.
	The last blank line before the next user's field, field, project or access grant is not
	considered part of this text.

## root Access Grant name
grant = this MUST always have a value, the rest except permissions can be empty.
tags =
description =
notes =
permissions =
- *
	/:

metadata =

```

The file doesn't contemplate to contain access grants of different accounts. If you want to store
access grants for different accounts, you MUST create a file for each one.

NOTE unless it's indicated in the field description, fields accept any number of UTF-8 character
despite Storj satellites may not. This is intentionally for not requiring changes in the parser if
Storj satellite changes its constraints in the future because the goal is storing data and the one
that reflects to the satellite has to previously exist.

Fields names and values are separated by equal (`=`) with ONLY one space on each side, unless that
it doesn't have a value (e.g. empty) or the value must start in the next line, then the space on the
right MUST not be present.

The file MUST be empty or start with a [project](#projects). A file with only blank lines or
starting with blank lines isn't valid.

### Projects

The files has to start with a project's introduced by a single hash (`#`) followed by one white
space.

It can contains multiple projects. Projects' names are unique and MUST have at least one character.

A project's name follows with a blank line.

It MUST contain at least one [access grant](#access-grant).

### Access grant

An access grant section follows a blank line preceded by a [project's name](#projects) or another
__access grant__.

An access grant section starts with two consecutive hashes (`##`) following one white space and the
access grant's name. Access grants' names are unique in the [project's scope](#projects) and MUST
have at least one character.

An access grant MUST have the in the specified order the following fields.

#### grant

The field MUST not be empty.

#### tags

A list of tags or empty if none.

A tag can ONLY contain lowercase letters, numbers, underscores (`_`), colons, back and forward
slashes (`\` and `/`).

Each tag is separated by comma (`,`) and a white space. Comma MUST only be present if there is a
next tag.

Tags must be in ONE line.

#### description

ONE LINE of free text. It can be empty.

I accepts any character except any new line control character.

#### notes

MULTI LINE free text.

Value starts in the next line.

It can contain new lines and empty new lines.

It must end with a new empty line, which isn't considered part of the value. If you want to have a
new empty line at the end of the value, add two.

#### permissions

List of permissions.

Each permission has a specific format and starts in a new line. The list of permissions has to end
with a new empty line, which marks the separation with the next field.

A permission has the following format:

Start with dash (`-`) following ONE white space and a bucket's name. Bucket's name MUST have ONE
character at least and when it only has one, it MUST NOT be star (`*`), and it cannot contain any
new line control character.

It follows with a new line and a tab followed by the path prefix and then colon (`:`).

A prefix MUST have at least ONE character. When it has one character and it's the forward slash
(`/`), it represents any prefix.

Paths can contain colons, the last colon in the line marks the end of the prefix and the separation
with the permissions for the prefix.

The permissions of a prefix is empty or a set (so, cannot contain each one more than once) formed by
at least one of the following words (in any specific order): `delete`, `list`, `read`, `write`.
Empty value means all the permissions.

#### metadata

List of user's metadata fields. It can be empty.

Each metadata field has a specific format and starts in a new line. The list of fields has to end
with a new empty line, which marks the separation with the next _[access grant](#access-grant)_ or
_[project](#projects)_ or a new field added in the future.

Each field starts with dash (`-`) following ONE white space following the field's name. Field's name
can contain any character except any new line control character and it MUST be at least one
character long.

Field's name follows a colon (`:`); if the field's name contains colons, the last
colon is considered the one that separates it from the field's value.

Fields value can contain any character. If it's a single line value it has to be next to the field's
name with a white space between the colon and the beginning of the value, in this case, it cannot
contains any new line control character.

Field values with new lines are accepted, but the field's name separator (`:`) follows with a new
line and a tab. Each new line part of the value must start with a tab. The first tab, is not
considered part of the value.
