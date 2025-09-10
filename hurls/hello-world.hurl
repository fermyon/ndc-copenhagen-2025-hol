# Check Hello NDC!
GET http://localhost:3000
HTTP 200
[Asserts]
body == "Hello NDC!"
