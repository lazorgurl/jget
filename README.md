# jget

```bash
$ echo '{"url": "http://httpbin.org/ip", "method": "GET", "body": "", "headers": {}}' | jget | jq .
{
  "status": 200,
  "headers": {
    "content-length": "33",
    "access-control-allow-origin": "*",
    "content-type": "application/json",
    "server": "gunicorn/19.9.0",
    "connection": "keep-alive",
    "access-control-allow-credentials": "true",
    "date": "Wed, 23 Nov 2022 23:35:52 GMT"
  }
}
```