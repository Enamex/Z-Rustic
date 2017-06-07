# Live Notes

## Problem Set 1

---

* [X] 1
* [?] 2
* [X] 3 (but unconvinced)
* [ ] 4

### P 3

Naive implementation increments twice. Getting 2 requests every time. Like:

```html
Received connection from: [127.0.0.1:61643]
Recieved request body:
GET / HTTP/1.1
Host: localhost:4414
Connection: keep-alive
Cache-Control: max-age=0
User-Agent: Mozilla/5.0 (Windows NT 6.3; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3107.4 Safari/537.36
Upgrade-Insecure-Requests: 1
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8
Accept-Encoding: gzip, deflate, br
Accept-Language: en-US,en;q=0.8
```

then, right after:

```
Connection terminates.
Received connection from: [127.0.0.1:61647]
Recieved request body:
GET /favicon.ico HTTP/1.1
Host: localhost:4414
Connection: keep-alive
Pragma: no-cache
Cache-Control: no-cache
User-Agent: Mozilla/5.0 (Windows NT 6.3; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3107.4 Safari/537.36
Accept: image/webp,image/apng,image/*,*/*;q=0.8
Referer: http://localhost:4414/
Accept-Encoding: gzip, deflate, br
Accept-Language: en-US,en;q=0.8
```

