format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet
run:
	cargo run 

release-arm:
	cargo lambda build --release --arm64

deploy:
	cargo lambda deploy

invoke:
	cargo lambda invoke --remote \
  		--data-ascii '{"String": "A prime number (or a prime) is a natural number greater than 1 that is not a product of two smaller natural numbers."}' \
  		--output-format json \
  		my-macro-polo-lambda

all: format lint test run

cargo lambda deploy \
  --iam-role arn:aws:lambda:us-east-1:376636494158:function:my-macro-polo-lambda