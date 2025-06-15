eval $(minikube docker-env)
docker build -t query-api:dev -f services/query-api/infra/Dockerfile .
docker build -t metrics-api:dev -f services/metrics-api/infra/Dockerfile .

