
# txlib
Text based epub library manager.

The program scans the provided root directory for epub files and creates a table with the following fields (read from the epub metadata):
+ Date (timestamp of date when book was added, YYMMDD format)
+ Title
+ Author
+ Pages (page count based on 2000 chars per page)
+ Series (blank by default)
+ Path

The table with the library data is: 
1. printed to stdout (for piping to other programs)
2. stored in a plain text file at `$HOME/.config/txlib/epub_db.txt`, that can be manually changed with your preferred text editor. Existing book information has to be edited this way.


A plain text library allows for easy extensibility and integration into any workflow. Some ideas are provided in the [Tips](#Tips) section.

Getting started: the first time the program is executed, the required config files will be created. However, to actually start using the program, a library path has to be provided in the config file `$HOME/.config/txlib/txlibrc`, pointing to the root folder to scan for epub files. 

Everytime the program runs, removed epub files are deleted from the generated table and new epub files are added.


## Usage

```
$ txlib [OPTIONS]

Options:
    -s, --sort <SORT>  sort by: date, title, author, pages or series [default: date]
    -r, --reverse      reverse sorting order
    -n, --no-save      print output without saving to DB
    -o, --open-db      open DB file (does not run the rest of the app)
    -h, --help         Print help
    -V, --version      Print version
```

## Tips
+ The delimiter for the table columns is `/`, however, the columns are also separated by double-spacing. Thus, if you require to differentiate the delimiter slash from the path slashes (e.g., when piping to another program), you can use `␣␣/` as the delimiter.
+ The ebook library can be queried with `fzf` while also limiting the search to certain fields. Like in the example below, where the path field is being excluded:
    ```
    $ txlib | fzf --reverse --delimiter="/" --nth=..5 --header-lines=1
    ```
