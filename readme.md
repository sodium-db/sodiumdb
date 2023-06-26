# SodiumDB

**This project is still in early development. Expect bugs, lack of features, and little optimization at this time.**

A blazingly fast 🚀, low overhead ⬇️, in-memory store built in Rust.\
A great fit for high read/low write operations with uncompromising read times.

### Features 📦
- Easy-to-use REST API out-of-the-box, powered by [Actix Web](https://actix.rs/)
- Authorization out-of-the-box
- Low overhead
- Simple set up with JSON files
- Blazingly Fast (Capable of several thousand req/s on all operations)
- Memory and Thread safety with the Mutex module
- Point-In-Time Snapshot System to *heavily* optimize I/O

### Benchmarks (Read Requests)
Using wrk w/ Sodium (6 Threads/200 Connections)
```
Running 1s test @ http://127.0.0.1:8080/read
  6 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.19ms  299.45us   4.59ms   91.48%
    Req/Sec    27.63k    12.42k  122.79k    98.36%
  167856 requests in 1.10s, 18.25MB read
Requests/sec: 152706.87
Transfer/sec:     16.60MB
```
Using redis-benchmark
```
ING_INLINE: 130039.02 requests per second
PING_BULK: 135685.22 requests per second
SET: 147492.62 requests per second
GET: 139470.02 requests per second
```
10-20% faster reads!\
Sodium's writes are still lagging behind Redis at about 100,000 req/s, but I am working to improve that.

### Getting started 🔎 (Subject To Change)
1. To get started, install the [Rust Compiler](https://www.rust-lang.org/)
2. Clone this repository to a desired directory.\
**The automatic setup function will do the rest for you. However, keep reading for manual configuration.**
3. Within the project directory, create a folder called dbs.
4. Within this folder, create 2 files; `settings.json` and `db.json`
5. In db.json, simply type `{}`. Or, you can input some JSON for the DB to read and start with.
6. In settings.json, follow this example:
```json
{
    "password": "myAmazingPassword",
    "address": "127.0.0.1",
    "port": 8080,
    "workers": 1,
    "snapshot_seconds": 30
}
```
workers is the amount of handlers that will process your requests; if you do not set it, it will be set to the number of physical cores on your machine.\
snapshot_seconds is the time between each snapshot (when data is written to disk) in seconds. Less time between snapshots can reduce performance. Defaults to 30.\
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