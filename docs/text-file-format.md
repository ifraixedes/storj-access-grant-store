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

The file uses the `.ags` extension and they are called AGS files.

The goal of having a strict format is that all the AGS files look equal in terms of syntax. The
advantages are:

- It helps of not having different ways for specifying the same thing and consequently not having to
  specify "linters".
- If they are tracked in a versioning system, the diffs are because of data being updated and not
  because of format being changed, which makes easier to see the changes.

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
	TThe last empty line before "permissions" field is not considered part of the notes.
permissions =
- bucket-name-1
	prefix1/
		read, write
	prefix2/
		all
	prefix3/
		write, read, delete, list
- bucket-name-2
	/
		all
metadata =
- field1
	this is the value
- field 2
	Spaces in the field name are valid
- field 3
	Multiple lines are OK. Each new line has to be indented by one tab, which are not
	considered part of the text.
	The last blank line before the next user's field, field, project or access grant is not
	considered part of this text.

## root readonly Access Grant name
grant = this MUST always have a value, the rest except permissions can be empty.
tags =
description =
notes =
permissions = list, read
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

A tag can ONLY contain lowercase letters, numbers, underscores (`_`), dashes (`-`), colons, back and
forward slashes (`\` and `/`).

Each tag is separated by comma (`,`) and a white space. Comma MUST only be present if there is a
next tag.

Tags must be in ONE line.

#### description

ONE LINE of free text. It can be empty.

I accepts any character except any new line control character.

#### notes

MULTI LINE free text.

Value starts in the next line and a tab (`\t`).

It can contain new lines and empty new lines, but all of them must start with a tab. The initial
tab isn't considered part of the text.

#### permissions

Denote the permissions of the access grant for the whole project or for specific buckets and paths.
Because of the levels of permissions, the syntax differ between both.

When denoting permissions for the whole project, after the equal sign it follows a space and the
rights (see below what rights are).

When specifying permissions for buckets and paths, each permission has the following format and
starts in a new line:

- Start with dash (`-`) following ONE white space and a bucket's name. Bucket's name MUST have ONE
character at least, and it cannot contain any new line control character.
- It follows with a new line and a tab followed by the path prefix.
- A prefix MUST have at least ONE character. When it has one character and it's the forward slash
(`/`), it represents any prefix.
- The path's rights are specified in the following line preceded by two consecutive tabs.

Rights are at least one of the following words: `all`, `delete`, `list`, `read`, `write`. Words can
be combined except the `all` one, which can be only set alone. When combined they must follow the
alphabetical order and separating the following one from the previous by a command and white space
(`, `).

#### metadata

List of user's metadata fields. It can be empty.

Each metadata field has a specific format and starts in a new line.

Each field starts with dash (`-`) following ONE white space following the field's name. Field's name
can contain any character except any new line control character and it MUST be at least one
character long and ends with a new line.

Every field must have a value.

A field's value is preceded by a tab followed by at lease one character. They can contain any
character. When they contain new lines, each new line start with a tab. The preceding first tab on
each line is not considered part of the value and the last new line character isn't considered part
of the field's value.
