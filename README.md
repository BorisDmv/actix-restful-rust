Crates used: actix-web,serde,serde_json

Routes:
- GET /api -> "Hello, world!"
- GET /api/data -> It will respond with a JSON object containing the data of the request.
- POST /api/echo -> It will respond with **data** of the request and **message** for signaling successfully submitted.
