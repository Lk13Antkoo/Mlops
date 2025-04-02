watch:
		cargo lambda watch
invoke:
		cargo lambda invoke --data-ascii '{"command": "this command has Marco"}' new-lambda-project
build:
		cargo lambda build --release --arm64
deploy:
		cargo lambda deploy --region us-east-1
aws-invoke:
		cargo lambda invoke --remote new-lambda-project --data-ascii '{"command":"this command has Marco"}'
	
