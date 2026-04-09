
### Setting up Tau and Asciinema with Caddy

This setup follows the [proxy-setup.md](docs/proxy-setup), however adds the
Asciinema server as an application running on the same server as your tau-tower
server. We need to reverse_proxy the asciinema-server, on default port 4000, to make the
server acessable. 

``` Caddyfile
# incoming audio source stream from tau-radio
example.com:8001 {
    reverse_proxy :6000
}

# outgoing audio broadcast stream from tau-tower
example.com {
    reverse_proxy :6001
}

# asciinema server
asciinema.example.com {
    reverse_proxy :4000
}
```

### Asciinema setup

Below is a copy from the [`Getting started`](https://docs.asciinema.org/getting-started/#self-hosting-the-server)-section from the Asciinema Documentation. 

----

While asciinema.org is the default asciinema server used by the CLI for uploading recordings, you can self-host your own instance if you want full ownership and control over the recordings.

asciinema server is packaged as OCI container image and is available at ghcr.io/asciinema/asciinema-server.

Here's a minimal docker-compose example:

```yml
services:
  asciinema:
    image: ghcr.io/asciinema/asciinema-server:latest
    ports:
      - '4000:4000'
    volumes:
      - asciinema_data:/var/lib/asciinema
    depends_on:
      postgres:
        condition: service_healthy

  postgres:
    image: docker.io/library/postgres:14
    volumes:
      - postgres_data:/var/lib/postgresql/data
    environment:
      - POSTGRES_HOST_AUTH_METHOD=trust
    healthcheck:
      test: ['CMD-SHELL', 'pg_isready -U postgres']
      interval: 2s
      timeout: 5s
      retries: 10

volumes:
  asciinema_data:
  postgres_data:
```

----

Start it with:

```bash
docker compose up
```

Then point asciinema CLI to it by setting ASCIINEMA_API_URL environment variable:
(Set env variable properly if the intended use is a public-facing asciinema server. 
Follow the full guide on https://docs.asciinema.org).

```
# export ASCIINEMA_API_URL=https://asciinema.example.com

# localhost example
export ASCIINEMA_API_URL=http://localhost:4000

asciinema rec demo.cast
asciinema upload demo.cast
```

Note that the above configuration should be used only for testing the server locally. See full [`server self-hosting`](https://docs.asciinema.org/manual/server/self-hosting/) guide to learn how to set it up properly in a full-featured and secure way.


