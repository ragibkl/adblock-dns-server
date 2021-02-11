# Running on k3s

K3s is a very lightweight kubernetes distribution. We can use k3s to setup a single node kubernetes cluster, and then use this to host our adblock dns server. Learn more about k3s [here](https://k3s.io/).

We will also make use of Keel. Keel will help us to automatically update the adblock dns when a new version is available.

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
kubectl apply -f 'https://sunstone.dev/keel?namespace=keel&username=admin&password=admin&tag=latest'
```

## Install the adblock server

Apply the deployment
```
kubectl apply -f adblock.yaml
```
