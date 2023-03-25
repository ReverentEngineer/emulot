# Guests API

* `/guests/create/<name>` - Creates a guest with the provided configuration
    - Method: `POST`
    - Request Body:
        ```
        {
            "arch": "i386",
            "memory": 512
        }
        ```
* `/guests/list` - Lists all the guest names mapped to their ID
    - Method: `GET`
    - Response Body:
        ```
        [
            {"guest-a": 1 },
            {"guest-b2": 2 }
        ]
        ```
* `/guests/remove/<id>`- Remove a guest configuration
    - Method: `DELETE`
* `/guests/start/<id>` - Start a guest
    - Method: `POST`
* `/guests/stop/<id>` - Stop a guest
    - Method: `POST`

