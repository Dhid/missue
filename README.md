# missue
A simple, local and personal issue tracker, work in progress.  
Made as a little project for learning the basics of Rust Programming Language.  
Based on the Rust course : https://github.com/pingcap/talent-plan.  
Any contribution is welcome!

### USAGE:  
    missue.exe <SUBCOMMAND>

### FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information

### SUBCOMMANDS:
    add    Add an issue with given options
    get    Get the issue, given the name
    ls     Lists all the open issues
    rm     Remove the issue, given the name

#### Add command
    -d, --description <description>    A string description of the issue
    -n, --name <name>                  A string name
    -s, --status <status>              Sets the status of the issue [possible values: OPEN, DOING, CLOSED]