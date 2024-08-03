##@ Utility
help:  ## Show this help.
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

start-db-service: ## start postgres db with docker-compose
	docker-compose -f docker-compose.yaml up

restart-db-service: clean-db-service ## delete and start postgres db with docker-compose
	docker-compose -f docker-compose.yaml up

clean-db-service: ## delete postgres db (containers + volumes)
	docker-compose stop
	docker container rm postgres_db
	docker volume prune -a

get_readyz_response: ## runs a get request to /readyz_endpoint
	curl -X GET localhost:8080/readyz

get_score_response: ## runs a post request to /score endpoint
	curl -X POST localhost:8080/score \
	-H 'Content-Type: application/json' \
	-H 'Accept: application/json' \
	-d '{"name": "taha", "age": 27, "job": "mlops", "salary": 10001}'

get_users_response: ## runs a post request to /users endpoint
	curl -X POST localhost:8080/users \
	-H 'Content-Type: application/json' \
	-H 'Accept: application/json' \
	-d '{"name": "John", "surname": "Doe", "id": "1234", "age": 30 , "profession": "mlops"}'

get_login_response: ## runs a post request to /login endpoint
	curl -X POST localhost:8080/login \
	-H 'Content-Type: application/json' \
	-H 'Accept: application/json' \
	-d '{"email": "Jamaican.brotha@gmail.com"}'