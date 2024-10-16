# lilac <img align="right" src="https://github.com/user-attachments/assets/d4aa1a75-e995-4d12-b298-a9525038c89c" alt="drawing"/>  
###### *A static site generator-esque tool for adding include statements to HTML (and other stuff).*

Lilac goes through all files in a directory, performs lexical tokenization and creates an abstract syntax tree.
Lastly, it parses that tree to create the final compiled file. If a file does not contain any lilac commands, it gets skipped and a hard link is created to the original file (this also applies to images and other media) as to not create duplicate files and save on space.

> [!NOTE]  
>  I mainly created this project with the intention of using it in my own future projects. When I started this, I was learning about compilers and wanted to create one of my own to understand them better.  
> If you are looking for a good, established static site generator, you might like [Eleventy](https://www.11ty.dev/), [jekyll](https://jekyllrb.com/) or [hugo](https://gohugo.io/) :)


## Installation
This requires you to have rust and cargo installed.
```bash
cargo install lilac
```
###### *I am willing to transfer ownership of the lilac crate name if you have a cool idea for it.*
Alternatively, you can clone this repo and build + install it locally.

## Commands
`lilac help` Prints a help message.  
`lilac init` Initiates the cwd as a lilac project.  
`lilac remove` Removes everything lilac related in the cwd.  
`lilac clean` Doesn't do anything at the moment.  
`lilac build` Compiles the cwd into `_lilac/build`  
`lilac run` Starts a local server with live-update.  

## Features
The following commands can be inserted into files and will be compiled by lilac: 

The **put** statement replaces itself with the contents of **file.txt** on compilation.  
```html
[[put path/to/file.txt]]
```
The **for** statement repeats the content in the for loop for each file in the directory **path/to/files**. For the *n-th* loop iteration, the variable **iterator** is set to the name of the *n-th* file in the directory.
```html
[[for path/to/files as iterator]]
    <p>test</p>
    [[put {iterator}]]
[[end]]
```
The **run** statement executes a file and replaces itself with the standard output.  
(shell scripts and js files (via node.js) currently supported)
```html
[[run path/to/script.sh]]
```

## (Sub) Sections
Files can be formatted in a way to create sections and subsections for partial includes. Section names are prepended by a colon and followed by the section contents. Subsections have one more colon than their immediate parent-section.
A tree structure is created with subsections containing subsections that can be traversed by **put** statements to include only parts of a file.

Let the following the contents of **file.txt**:
```
:first_section
The sun is a mass of incandescent gas,
::first_subsection
a gigantic nuclear furnace.
::second_subsection
Where hydrogen is built into helium at a temperature of millions of degrees.
:second_section
::a
Yo-ho, it's hot, the sun is not a place where we could live.
:::a.a
But here on Earth, there'd be no life without the light it gives.
:::a.b
We need its light, we need its heat.
::b
We need its energy.
::c
Without the sun, without a doubt. There'd be no you and me.
```
Then `[[put file.txt:first_section]]` would be compiled into `The sun is a mass of incandescent gas,`  
`[[put file.txt:first_section:first_subsection]]` into `a gigantic nuclear furnace.`  
and `[[put file.txt:second_section:a:a.b]]` into `We need its light, we need its heat.`

**for** statements can iterate over (sub)sections instead of files in a folder.
```html
[[for file.txt:second_section as content]]
  [[put {content}]]
[[end]]
```
would compile into
```html
Yo-ho, it's hot, the sun is not a place where we could live.
We need its energy.
Without the sun, without a doubt. There'd be no you and me.
```
note that `file.txt:second_section:a.a` and `file.txt:second_section:a.b` are not included because they are not immediate children of `file.txt:second_section`.

If you want to include the title of a section, use the `;title` suffix.
`[[put file.txt:second_section:a;title]]` would compile into `a`. Not so useful unless you use it with a **for** statement:
```html
[[for file.txt:second_section as content]]
  [[put {content};title]]
[[end]]
```
would compile into
```html
a
b
c
```
Use the `;<n>` suffix to get the *n-th* child of a subsection.  
`[[put file.txt:second_section;0]]` compiles into `Yo-ho, it's hot, the sun is not a place where we could live.`
## Parameters and Variables
**run** statements can contain parameters. `[[run script.sh arg1 arg2]]` will run **script.sh** with `arg1` as the first argument and `arg2` as the second argument.  
**put** statements can contain parameters as well. `[[put text.txt arg1 arg2]]` will replace all occurrences of `{$0}` with `arg1` and all occurrences of `{$1}` with `arg2` in **text.txt**.
## Example
How to create a self-generating RSS-Feed with lilac:
```html
<?xml version=1.0 encoding=UTF-8 ?>
<rss version=2.0>

<channel>
  <title>my RSS feed</title>
  <link>https://example.com</link>
  <description>news and more</description>
  <copyright>Copyright 2024, Me<\\copyright>
  [[for newsfeed.txt as news]]
    <item>
      <title>[[put {news};title]]</title>
      <link>https://example.com/news/[[put {news};title]]</link>
      <description>[[put {news}:description]]</description>
      <author>name@email.com</author>
    </item>
  [[end]]
</channel>

</rss>
```
## Used Crates
[clap](https://github.com/clap-rs/clap) for the CLI interface  
[notify](https://github.com/notify-rs/notify) for listening to changes in the file structure, so I can live-reload the page  
[tungstenite](https://github.com/snapview/tungstenite-rs) for simple websockets  
[walkdir](https://github.com/BurntSushi/walkdir) for traversing directories

and the usual suspects: `serde`, `toml` and `regex`
