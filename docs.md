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

## Document System
SodiumDB now supports a document-like system. As in, you can query a "document" (a key whose value is a JSON Object) and query a key within the object.\
In create you can simply set your value to a JSON object. However, you are now able to set up your "/delete" and "/read" bodies a little differently.

## /delete
In "/delete" you can set up an optional "doc" key to query a specific object.\
If you'd like to delete a full object, simply omit "doc" and have "entry" be the name of the object's key.\
Example:
```json
{"entry": "hello", "doc": "hello"}
```
Deletes the selected entry if it exists, and returns the key/value pair of the deleted element.\
Returns a 400 Error if it does not exist.

## /read
In "/read" you can set up an optional "doc" key to query a specific object.\
If you'd like to read a full object, simply omit "doc" and have "entry" be the name of the object's key.\
Example:
```json
{"entry": "hello", "doc": "hello"}
```
Returns a json body like this: `{"result": "world"}`.\
Like "/delete", returns a 400 Error if the selected key does not exist.