**REST API Documentation Page / Guide**

**For now, all endpoints (except for the index, which uses GET) only allow for POST methods.**

If entries are maxed out via your entry_limit setting, 400s will be returned.

## Header Guide
Set the HTTP Headers to the following format:
```json
{
    "Authorization": "myAmazingPassword",
    "Content-Type": "application/json"
}
```

Unauthorized requests return a 401 Unauthorized error.

## /create
"/create" accepts any valid JSON body.\
Example:
```json
{"hello": "world"}
```
This body will be inserted into the DB.\
Returns the body you sent.

## /delete
"/delete" must be formatted like so.
Example:
```json
{"entry": "hello"}
```
Deletes the selected entry if it exists, and returns the key/value pair of the deleted element.\
Returns a 400 Error if it does not exist.

## /read
"/read" is formatted just like "/delete".
Example:
```json
{"entry": "hello"}
```
Returns a json body like this: `{"result": "world"}`.\
Like "/delete", returns a 400 Error if the selected key does not exist.