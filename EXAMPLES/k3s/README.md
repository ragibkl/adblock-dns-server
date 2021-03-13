# Running on k3s

K3s is a very lightweight kubernetes distribution. We can use k3s to setup a single node kubernetes cluster, and then use this to host our adblock dns server. Learn more about k3s [here](https://k3s.io/).

We will also make use of Keel. Keel will help us to automatically update the adblock dns when a new version is available. Learn more about keel [here](https://keel.sh/).

# Current Progress

At the moment, I managed to get the pod and service to be up an running. the service binds on port 53 on the host, and the pod can auto-updates without down time. However, due to k3s limitation, the service only listens to `ipv4`, and cannot bind to `ipv6`. I haven't figured out how work around this limitation.

Logs are also not yet working as I haven't figured out how to export logs in this environment.

- [x] dns pod up and running
- [x] dns update with no downtime
- [x] dns binds to port 53 on host
- [x] dns listens to ipv4
- [] dns listens to ipv6
- [] dns debug logs
- [] dns receives actual client ip

# Setting up

## Disable Systemd-Resolved

See [here](https://medium.com/@niktrix/getting-rid-of-systemd-resolved-consuming-port-53-605f0234f32f)

## Install k3s

Install k3s on your node. See [here](https://rancher.com/docs/k3s/latest/en/quick-start/).
```
curl -sfL https://get.k3s.io | sh -
```

We need to allow for service ports outside of the normal range. Open the `k3s.service` file.

```shell
nano /etc/systemd/system/k3s.service
```

We need to add some arguments for the kube apiserver to allow for low service ports range.

```
# file: /etc/systemd/system/k3s.service
...
ExecStart=/usr/local/bin/k3s \
    server \
    '--kube-apiserver-arg' 'service-node-port-range=1-32767' \
```

Reload the systemd daemon and restart k3s.
```
systemctl daemon-reload
systemctl restart k3s
```

## Install keel

Keel will help auto update our adblock dns server. Apply the deployment.
```
kubectl apply -f 'https://sunstone.dev/keel?namespace=keel'
```

If you want to see the admin dashboard of keel, use the following instead
```
kubectl apply -f 'https://sunstone.dev/keel?namespace=keel&username=admin&password=admin&tag=latest'
```

The dashboard can be accessed at `http://[YOUR_SERVER_IP]:9300`


# Run the adblock server

1. Before proceeding any further, open the `adblock.yaml` file, and inspect the ConfigMap section. Feel free to edit the values as you see fit.

2. Apply the deployment
```
kubectl apply -f adblock.yaml
```
