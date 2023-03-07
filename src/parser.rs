use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ags.pest"]
pub(crate) struct AGS;

#[cfg(test)]
mod tests {
    use super::*;

    use pest::Parser;

    #[test]
    fn test_ags_pest_parse_one_project_one_access_all_filled() {
        let unparsed_file = r#"# my project

## my access
grant = my grant value encrypted
tags = one, two
description = this is my access grant for ...
notes =
	This is a note.
	Notes support multiline text.
permissions =
- my-bucket
	wow
		read, write
metadata =
- field 1
	wow
- field 2
	Multiple lines are OK. Each new line has to be indented by one tab, which are not
	considered part of the text.
	The last blank line before the next user's field, field, project or access grant is not
	considered part of this text.

"#;

        AGS::parse(Rule::file, unparsed_file).expect("unsuccessful parse");
    }

    #[test]
    fn test_ags_pest_parse_one_project_one_minimal_access_grant() {
        let unparsed_file = r#"# my project

## my access
grant = my grant value encrypted
tags =
description =
notes =
permissions = all
metadata =

"#;

        AGS::parse(Rule::file, unparsed_file).expect("unsuccessful parse");
    }

    #[test]
    fn test_ags_pest_parse_one_project_more_than_one_access() {
        let unparsed_file = r#"# my project

## my-bucket full access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		all
metadata =
- field 1
	TBD

## my root write access
grant = my grant value encrypted
tags = one
description =
notes =
	This is a note.
permissions = write
metadata =
- app
	web3
- clients
	- Client A
	- Client B
	- Client C

"#;

        AGS::parse(Rule::file, unparsed_file).expect("unsuccessful parse");
    }

    #[test]
    fn test_ags_pest_parse_more_than_one_project() {
        let unparsed_file = r#"# my project 1

## my-bucket full access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		all
metadata =
- field 1
	TBD

## my-bucket full access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		all
metadata =
- field 1
	TBD

## my readonly root access proj1
grant = my grant value encrypted
tags = one
description =
notes =
	This is a note.
permissions = list, read
metadata =
- app
	web3
- clients
	- Client A
	- Client B
	- Client C

# my project 2

## my-bucket access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		list, read, write
metadata =

## my root access proj2
grant = my grant value encrypted
tags = one, root-access
description = this is my access grant for ...
notes =
	This is a note.
	Notes support multiline text.
permissions = all
metadata =
- field 1
	TBD
- field 2
	Multiple lines are OK. Each new line has to be indented by one tab, which are not
	considered part of the text.
	The last blank line before the next user's field, field, project or access grant is not
	considered part of this text.

"#;

        AGS::parse(Rule::file, unparsed_file).expect("unsuccessful parse");
    }

    #[test]
    fn test_ags_pest_parse_one_project_with_description() {
        let unparsed_file = r#"# my project

This is the description of this project
in multiple lines

## my-bucket full access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		all
metadata =
- field 1
	TBD

## my root write access
grant = my grant value encrypted
tags = one
description =
notes =
	This is a note.
permissions = write
metadata =
- app
	web3
- clients
	- Client A
	- Client B
	- Client C

"#;

        AGS::parse(Rule::file, unparsed_file).expect("unsuccessful parse");
    }

    #[test]
    fn test_ags_pest_parse_more_than_one_project_with_descripion() {
        let unparsed_file = r#"# my project 1

## my-bucket full access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		all
metadata =
- field 1
	TBD

## my-bucket full access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		all
metadata =
- field 1
	TBD

## my readonly root access proj1
grant = my grant value encrypted
tags = one
description =
notes =
	This is a note.
permissions = list, read
metadata =
- app
	web3
- clients
	- Client A
	- Client B
	- Client C

# my project 2

It has a single line description

## my-bucket access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		list, read, write
metadata =

## my root access proj2
grant = my grant value encrypted
tags = one, root-access
description = this is my access grant for ...
notes =
	This is a note.
	Notes support multiline text.
permissions = all
metadata =
- field 1
	TBD
- field 2
	Multiple lines are OK. Each new line has to be indented by one tab, which are not
	considered part of the text.
	The last blank line before the next user's field, field, project or access grant is not
	considered part of this text.

# my project 3

It has a multi line
description

## my-bucket access
grant = my grant value encrypted
tags = one, two
description =
notes =
permissions =
- my-bucket
	/
		list, read, write
metadata =

"#;

        AGS::parse(Rule::file, unparsed_file).expect("unsuccessful parse");
    }
}
