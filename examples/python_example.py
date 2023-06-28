import requests

headers = {"Authorization": "myAmazingPassword", "Content-Type": "application/json"}
url = 'http://127.0.0.1:8080'

# create
requests.post(f"{url}/create", headers=headers, json={"hello": "world"})

# read
read = requests.post(f"{url}/read", headers=headers, json={"entry": "hello"})
assert read.json()["result"] == "world"

# delete
delete = requests.post(f"{url}/delete", headers=headers, json={"entry": "hello"})
assert delete.json() == {"hello": "world"}