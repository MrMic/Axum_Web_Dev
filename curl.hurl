# Get hello "World"
GET http://localhost:8080/hello

# Get hello "Mic"
GET http://localhost:8080/hello?name=Mic

# Get hello "Mike"
GET http://localhost:8080/hello2/Mike

# POST /api/login
POST http://localhost:8080/api/login
{
  "username": "demo1",
  "pwd": "welcome"
}
HTTP 200
[Asserts]
cookie "auth-token" == "user-1.exp.sign"

GET http://localhost:8080/hello2/Mike
# Get hello "World"
