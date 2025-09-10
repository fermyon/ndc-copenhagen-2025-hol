# Check spinframework.dev
GET https://spinframework.dev
HTTP 200
[Asserts]
header "content-type" == "text/html; charset=utf-8"
body contains "Introducing Spin"
