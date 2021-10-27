### API
- Healthy check: 'GET' /

    curl http://localhost:8000/healthy

- Register User: 'POST' /

    curl --request POST \
      --url http://localhost:8000/auth/signup \
      --header 'content-type: application/json' \
      --data '{
        "username": "user2",
        "email": "user2@example.com",
        "password": "user1"
      }'