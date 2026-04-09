
## To use Tau-Radio with Tau-tower on a remote server

----

### Setting up Tau with Caddy


We use `Caddy` to funnel the source client stream to the broadcast software
[tau-tower](https://github.com/tau-org/tau-tower). 

[How to install Caddy on your system](https://caddyserver.com/docs/install)

Make sure that your VPS or remote server has the correct ports open to receive
and broadcast the radio stream. 

#### TLS __enabled__ source stream
``` Caddyfile
# incoming audio source stream
example.com:8001 {
    reverse_proxy :6000
}

# outgoing audio broadcast stream
example.com {
    reverse_proxy :6001
}
```

---

The above Caddyfile assumes that `tau-radio` has enabled __tls/ssl encryption__, but for if for
some reason you would like to broadcast with __tls/ssl__ disabled, you can
configure Caddy like below:

If doing this, __it would be wise to restrict your server to only receive from your public IP 
when sending to the upstream port__. 

#### TLS __disabled__ source stream
```Caddyfile
# incoming INSECURE audio source stream
# use the IP address of the upstream server instead of domain name
:8001 {
    reverse_proxy :6000
}

# outgoing audio broadcast stream
example.com {
    reverse_proxy :6001
}
```

---

## TOML configs

The config files below, should be placed in a `$HOME/.config/tau` or `$XDG_CONFIG_HOME/tau` directory. 
- `tau-radio` - `config.toml`
- `tau-tower` - `tower.toml`


Below is an updated config file for `tau-tower` which uses the ports from `Caddyfile` example [above](#tls-enabled-source-stream). 

``` tower.toml
username = "username" 
password = "emanresu" 

# local listen port
listen_port = 6000

# local broadcast port
broadcast-port = 6001      

broadcast-endpoint = "tau.ogg"       

# When Asciinema server is used, we need to allow it to fetch the stream
cors_allow_list = ["https://asciinema.example.com"]
```

In the `tau-radio` client config, we set `tls = true` enabling a __tls/ssl-encrypted__ 
stream to the remote server on the port we have set. Caddy will handle the decryption if 
configured as the TLS enabled Caddyfile example
[above](#tls-enabled-source-stream).

``` config.toml
username = "username"
password = "emanresu"

# URL to the remote server ( use IP address if tls = false )
url = "example.com"

# The remote server port where we send the stream
upstream_port = 8001

# Default audio interface on macOS
audio_interface = "BlackHole 2ch"

# On linux
# audio_interface = "pipewire"

# broadcast behind tls/ssl encryption ( recommended )
tls = true
```
