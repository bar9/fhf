# File Header Filler (from git history)

## Overview
For a code repository, this automatically adds file headers to PHP code
files based on author and company information 
extracted from Git history. 
This tool streamlines the process of ensuring that each PHP code file 
contains the necessary header information for 
legal compliance and accountability purposes.

Note: Currently, File Header Filler only supports PHP files.

## Installation
Currently, the only way to install fhf is to build it from source.

To install File Header Filler, you can use the following script:
```bash
cargo install --git https://github.com/bar9/fhf
```

## Example Header
An example header added by File Header Filler looks like this:
```php
/**
* @author Roland Brand / acme inc.
*
* @since 04/2024
*/
```

## Example Usage
To utilize File Header Filler, you can use the following command-line syntax:

```bash
fhf --path . --extension php --suffix 'acme inc.' --ignore node_modules,vendor,.idea,bin
```

This command will search for PHP code files within the specified path
(`.` denotes the current directory)
and add the file header information with the suffix "acme inc."
The tool will ignore specified directories such as node_modules, vendor, .idea, and bin.

Note: File Header Filler only adds headers to files that do not have a block comment at the start of the file.


Note: Always review and verify the changes made by File Header Filler
to ensure compliance 
with your organization's policies and legal requirements.

