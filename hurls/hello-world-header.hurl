# Check Hello NDC Header!
GET http://localhost:3000
HTTP 200
[Asserts]
body == "Hello NDC!"
header "X-Spin" == "true"
