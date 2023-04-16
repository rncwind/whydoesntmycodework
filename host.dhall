let HostConfig
    {- Define our host type as two text fields and a number -}
    : Type
    = { hostname : Text, ip_addr : Text, port : Natural }

let makeHost =
    {- Given a hostname, an IP and a port, produce a Host type -}
      λ(hostname : Text) →
      λ(ip_addr : Text) →
      λ(port : Natural) →
        let host
            : HostConfig
            = { hostname, ip_addr, port }

        in  host

let configs
    {- Our configs are a list of multiple Host types. -}
    : HostConfig
    = makeHost "localhost" "0.0.0.0" 3000

in  configs
