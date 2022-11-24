# jget

```bash
$ jo url=https://httpbin.org/ip method=GET | jget | jq '.body | fromjson'
{
  "origin": "172.92.128.126"
}
$ jo url=https://httpbin.org/ip method=GET | jget | jq .
{
  "status": 200,
  "body": "{\n  \"origin\": \"172.92.128.126\"\n}\n",
  "headers": {
    "date": "Thu, 24 Nov 2022 03:16:09 GMT",
    "connection": "keep-alive",
    "server": "gunicorn/19.9.0",
    "access-control-allow-credentials": "true",
    "access-control-allow-origin": "*",
    "content-type": "application/json",
    "content-length": "33"
  }
}
```