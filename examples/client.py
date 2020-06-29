import requests
import json

data = {"image_path":"https://c-ssl.duitang.com/uploads/item/201412/11/20141211205355_W23ER.jpeg"}
headers = {"Content-Type": "application/json"}
res = requests.post("http://127.0.0.1:9527/api/v1/search", data=json.dumps(data), headers=headers)
print(res.json())