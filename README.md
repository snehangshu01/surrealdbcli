# surrealdbcli
This is a custom cli for SurrealDB.
It is entirely built in Rust.

## The Story
Very recently I started learning & experimenting with Rust owing to its performance attributes.

I also fell interested towards "SurrealDB" a multi-modal database that combines features of RDBMS, DocumentDB, NoSQL and GraphDB combined into one.
Soon I was looking for a CLI to help me practice. 

When I wanted  a CLI to play with SurrealDB, I didn't know that there was one.
The SurrealDB's online documentation suggests to install with npm or yarn.
When I went to "npmjs" website, it did have some options which was kinda made me confused and so was a dead end.

Only When I finished this "POC" that I created and went to find some SurrealDB query example, then I found the "surreal sql" command under "CLI Tool > SQL Command > Example Usage". This was the CLI I was looking for.

Tried it & compared it with my CLI's output.
I see that my CLI gives me the Stats & and also gives a JSON Output.
Thought it might help. And hence the upload.

## Benefits
* Outputs in JSON Format
* Includes the performance stats in the output.


## Bibliography
The Sample of DB Connection was taken from : https://docs.surrealdb.com/docs/integration/sdks/rust

