let Ports
    : Type
    = { http : Natural, https : Natural }

let HostConfig
    {- Define our host type as two text fields and a number -}
    : Type
    = { hostname : Text, ip_addr : Text, ports : Ports }

let makeHost =
    {- Given a hostname, an IP and a port, produce a Host type -}
      λ(hostname : Text) →
      λ(ip_addr : Text) →
      λ(ports : Ports) →
        let host
            : HostConfig
            = { hostname, ip_addr, ports }

        in  host

let makePorts =
      λ(http : Natural) →
      λ(https : Natural) →
        let ports
            : Ports
            = { http, https }

        in  ports

let configs
    {- Our configs are a list of multiple Host types. -}
    : HostConfig
    = makeHost "localhost" "0.0.0.0" (makePorts 8080 3000)

in  configs
