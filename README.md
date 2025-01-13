<img src="https://github.com/W0nchichi/images/blob/main/pinkdarkluma-icon.png" />

I've just set some basic framworking for my project here, trying to follow this through


### <mark style="background: #BBFABBA6;"><b>Front-End: </b></mark>
The client will connect to a login prompt, and if it exists, they'll be granted access into the repl
- repl.rs:
	- [x] Prompt the user for input in an interactive loop.
	- [x] Send user commands to the server for processing.
	- [x] Display server responses in a user-friendly format.
	- [ ] Handle errors, such as invalid inputs or server disconnections.

- client.rs:
	- [x] Establish a TCP connection with the server.
	- [x] Implement authentication by sending a username and password to the server.
	- [x] Send commands to the server over the connection.
	- [x] Receive and deserialize server responses.
	- [ ] Reconnect automatically if the connection is lost. (idk how)

### <mark style="background: #FF5582A6;"><b>Back-End:</b></mark>

- server.rs:
	- [x] Listen for incoming client connections.
	- [x] Authenticate clients based on provided username and password.
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
	- [x] Tokenize raw input strings into meaningful symbols.
	- [x] Identify and handle SQL keywords like SELECT, FROM, WHERE, etc.
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

# File Structure:
Updated for scalability
```bash
luma_db 📂
├── lumadb_client 📂
|   ├── src 📂
|   |   ├── repl.rs         # Handles REPL logic.
|   |   ├── lib.rs         
|   │   └── client.rs       # Establishes client connection to the server.
|   └── Cargo.toml  
├── lumadb_server 📂
|   ├── src 📂       
|   |   ├── lib.rs         
|   │   └── server.rs       # Establishes server.
|   └── Cargo.toml  
├── lumadb_core 📂
|   ├── src 📂
|   |   ├── lib.rs
|   |   ├── tokenizer.rs    # Tokenizes SQL-like input.
|   │   ├── token.rs        # token enums.
|   │   ├── ast.rs          # Defines Abstract Syntax Tree structures.
│   |   ├── datatypes.rs    # In-memory and file-based storage.
|   │   └── lumadb.lalrpop.rs  # parsing stuff
|   └── Cargo.toml  
├── lumadb 📂
|   ├── src 📂
|   |   ├── lib.rs
|   |   └── config.rs        # Config settings for the database.
|   └── Cargo.toml  
├── src 📂
├── cargo.toml
├── build.rs
├── lib.rs
├── main.rs
├── mod.rs
└── README.md
```
