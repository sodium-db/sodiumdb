# SodiumDB

**This project is still in early development. Expect bugs, lack of features, and little optimization at this time.**

A blazingly fast 🚀, low overhead ⬇️, in-memory store built in Rust.

### Features 📦
- Frontend REST API out-of-the-box, powered by [Actix Web](https://actix.rs/)
- Authorization out-of-the-box
- Low overhead (Potentially under 500kb RAM Usage at startup)
- Simple set up with JSON files
- Blazingly Fast (Capable of several thousand req/s on all operations)
- Memory and Thread safety with the Mutex module

### Getting started 🔎 (Subject To Change)
1. To get started, install the [Rust Compiler](https://www.rust-lang.org/)
2. Clone this repository to a desired directory.
3. Within the project directory, create a folder called dbs.
4. Within this folder, create 2 files; `settings.json` and `db.json`
5. In db.json, simply type `{}`. Or, you can input some JSON for the DB to read and start with.
6. In settings.json, follow this example:
```json
{
    "password": "myAmazingPassword",
    "address": "127.0.0.1",
    "port": 8080
}
```
7. Navigate to the project directory and run `cargo run --release` in the command line. (Do not worry about compile time or a somewhat large binary, this is normal)

### Authorization 🔒 (Subject To Change)
1. Pick your favorite HTTP Client to get started. This can be something like Postman, requests for Python, fetch in Node.js, or whatever you prefer.
2. In the client set the request header using the following example:
```json
{"Authorization": "myAmazingPassword"}
```

### Interaction with the DB 🗣️ (Subject To Change)
As of now, Sodium has 3 endpoints; `/create`, `/read` and `/delete`, all of which use the POST method.

Create requires a JSON body. Simply add a JSON body to your request and it'll write it to the database.\
Read and Delete are very similar. Follow the following example:
```json
{"entry": "entry_to_get_or_delete"}
```
Read will get the value of the entry if it exists, and Delete will delete it.

**Here is a basic read request using Python:**
```python
r = requests.post('http://127.0.0.1:8080/read', headers={"Authorization": "myAmazingPassword"}, json={"entry": "hello"})
print(r.json()) # prints the value of key 'hello' if it exists
```

That's the guide for now. **Any Questions?** Open an issue, or contact me on discord @rainydevzz <3