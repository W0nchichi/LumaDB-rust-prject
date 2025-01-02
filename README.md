<img src="https://github.com/W0nchichi/images/blob/main/pinkdarkluma-icon.png" />

I've just set some basic framworking for my project here, trying to follow this through


### <mark style="background: #BBFABBA6;"><b>Front-End: </b></mark>
The client will connect to a login prompt, and if it exists, they'll be granted access into the repl
- repl.rs:
	- [ ] Prompt the user for input in an interactive loop.
	- [ ] Send user commands to the server for processing.
	- [ ] Display server responses in a user-friendly format.
	- [ ] Handle errors, such as invalid inputs or server disconnections.

- client.rs:
	- [ ] Establish a TCP connection with the server.
	- [ ] Implement authentication by sending a username and password to the server.
	- [ ] Send commands to the server over the connection.
	- [ ] Receive and deserialize server responses.
	- [ ] Reconnect automatically if the connection is lost. (idk how)

### <mark style="background: #FF5582A6;"><b>Back-End:</b></mark>

- server.rs:
	- [ ] Listen for incoming client connections.
	- [ ] Authenticate clients based on provided username and password.
	- [ ] Dispatch received commands to the appropriate handler.
	- [ ] Send responses back to the client.
	- [ ] Log all client inputs.

- handler.rs:
	- [ ] Parse received commands and route them to the corresponding functions.
	- [ ] Handle database queries such as SELECT and CREATE operations.
	- [ ] Respond with errors for invalid commands or unauthorized actions.

- auth.rs:
	- [ ] Validate username and password against a predefined set of credentials.
	- [ ] Implement basic hashing for password security.
	- [ ] Reject unauthorized login attempts.

- storage.rs:
	- [ ] Turn raw strings into columnar format for efficient storage.
	- [ ] Store columnar data in memory for quick access.
	- [ ] Persist data to disk in a simple format (e.g., CSV or JSON).
	- [ ] Retrieve stored data based on query parameters.
	- [ ] Implement basic indexing for faster data lookups.

- tokenizer.rs:
	- [ ] Tokenize raw input strings into meaningful symbols.
	- [ ] Identify and handle SQL keywords like SELECT, FROM, WHERE, etc.
	- [ ] Preserve syntactic rules during tokenization.
	- [ ] Generate tokens with positional metadata for error reporting.

- parser.rs:
	- [ ] Convert tokens into a structured Abstract Syntax Tree (AST).
	- [ ] Validate the syntax of SQL-like queries.
	- [ ] Provide meaningful error messages for invalid queries.
	- [ ] Integrate with the `.lalrpop` file for grammar definitions.

- ast.rs:
	- [ ] Define data structures for the Abstract Syntax Tree.
	- [ ] Support common SQL operations like SELECT, INSERT, and CREATE.
	- [ ] Represent conditions and expressions as nested structures.
	- [ ] Provide methods to traverse and manipulate the AST.

- query_executor.rs:
	- [ ] Execute SQL-like queries on in-memory and persisted data.
	- [ ] Process SELECT queries by filtering and projecting data.
	- [ ] Handle CREATE queries to define new tables.
	- [ ] Support basic aggregate functions like COUNT and SUM.
	- [ ] Return query results in a structured format.

- log_file.rs:
	- [ ] Record all client commands and server responses.
	- [ ] Write logs to a file for persistence.
	- [ ] Support log rotation to prevent file size issues.
	- [ ] Provide a utility to search and filter log entries.

- in_memory_table.rs:
	- [ ] Store temporary table data in memory.
	- [ ] Allow modification of table data during queries.
	- [ ] Handle table cleanup after client disconnects.
	- [ ] Provide methods for table joins and filtering.

- command.rs:
	- [ ] Define supported commands as enums for type safety.
	- [ ] Serialize and deserialize commands for client-server communication.
	- [ ] Map command enums to their respective handler functions.
	- [ ] Extend support for additional commands in the future.

- lumadb.rs:
	- [ ] Initialize the database and load configuration settings.
	- [ ] Manage the lifecycle of in-memory and persisted tables.
	- [ ] Act as the central interface for all database operations.
	- [ ] Provide APIs for server and handler interaction.

# File Structure and responsibilities:
```bash
rust-database-project 📂
├── lumadb-client 📂
|   ├── repl.rs         # Handles REPL logic.
│   └── client.rs       # Establishes client connection to the server.
├── lumadb-server 📂
│   ├── server.rs       # Main server logic.
│   ├── handler.rs      # Handles client commands.
│   └── auth.rs         # Authentication logic.
├── lumadb-core 📂
│   ├── tokenizer.rs    # Tokenizes SQL-like input.
│   ├── parser.rs       # Creates AST from tokens.
│   ├── ast.rs          # Defines Abstract Syntax Tree structures.
│   ├── storage.rs      # In-memory and file-based storage.
│   └── query_executor.rs  # Executes queries on in-memory or persisted data.
├── lumadb 📂
|   ├── lumadb.rs        # Main database instance.
|   ├── in_memory_table.rs # Holds in-memory tables.
|   ├── log_file.rs      # Logs transactions.
|   ├── command.rs       # Defines supported commands.
|   └── config.rs        # Config settings for the database.
├── cargo.toml
└── README.md
```
## lumadb-client 📂
### <mark style="background: #D2B3FFA6;">client.rs</mark>
The purpose of client.rs is to establish a link to the server which is running luma-db. Once it establishes the connection, the user will be asked to enter a username, and password. If these are recognized by the server, the user will be granted access to the Repl, which will allow them to send inputs to the server which holds all of the tables and such.

I can either use 

### <mark style="background: #FFB8EBA6;">repl.rs</mark>
Since all the repl is doing is asking the user to complete a string over a period of time, we can build it into the client side, and send the string over to the server for processing when the user is finished entering. The repl becomes useable once the client.rs is done authenticating and such

## lumadb-server 📂
### <mark style="background: #ADCCFFA6;">server.rs</mark>
The Server.rs file should interact with everything in the luma-db file, opening the connection to a client.rs file which begins the connection. It should instantiate the database and for now, only handle one connection.

## lumadb 📂
### <mark style="background: #ABF7F7A6;">lumadb.rs</mark>
This is the instance of the database itself, which handles the calling of other .rs files when needed by the server.rs file, retrieving tables and such

### <mark style="background: #BBFABBA6;">in_memory_table.rs</mark>
The in_memory_table file is responsible for holding a table that has been queried by the user, which is a non-permanent table unlike the ones stored on disk

### <mark style="background: #FFF3A3A6;">log_file.rs</mark>
The log file literally just keeps track of the transactions and such happening and writes it to a file

### <mark style="background: #FFB86CA6;">command.rs</mark>
This file keeps track of the commands the user is able to execute, right now, select and create are the only I can realistically impliment

### <mark style="background: #FF5582A6;">lumadb.lalrpop</mark>
This file uses the .lalrpop parser to handle syntax trees from the tokenizer 

### <mark style="background: #CACFD9A6;">parser.rs</mark>
what interacts with the .lalrpop file in order to take the input from the other .rs files
