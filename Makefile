aws_role := arn:aws:iam::700349367502:role/service-role/testLambdaRole
function_name := testLambda
target := x86_64-unknown-linux-gnu

build:
	cargo build --release --target $(target)

prepare_deploy: build
	cp ./target/$(target)/release/test_lambda ./bootstrap && \
	zip lambda.zip bootstrap && \
	rm bootstrap

first_deploy: prepare_deploy
	aws lambda create-function \
		--function-name $(function_name) \
		--handler doesnt.matter \
		--zip-file fileb://./lambda.zip \
		--runtime provided.al2 \
		--role $(aws_role) \
		--environment Variables={RUST_BACKTRACE=1} \
		--tracing-config Mode=Active

deploy: prepare_deploy
	aws lambda update-function-code \
		--function-name $(function_name) \
		--zip-file fileb://./lambda.zip