kubectl create configmap query-api-config --from-file=services/query-api/config/dev.toml
kubectl apply -f services/query-api/infra/k8s/deployment.yaml
kubectl apply -f services/query-api/infra/k8s/service.yaml

kubectl create configmap metrics-api-config --from-file=services/metrics-api/config/default.toml
kubectl apply -f services/metrics-api/infra/k8s/deployment.yaml
kubectl apply -f services/metrics-api/infra/k8s/service.yaml

