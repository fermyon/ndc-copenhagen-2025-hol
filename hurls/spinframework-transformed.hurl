# Check Proxy-Transformed Header
GET http://localhost:3000
HTTP 200
[Asserts]
header "content-type" == "text/html; charset=utf-8"
body contains "Introducing Spin"
header "X-Transformed" == "true"
