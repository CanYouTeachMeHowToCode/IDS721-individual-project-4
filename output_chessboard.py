import json
f = open('output.json')
data = json.load(f)
print(data["msg"])