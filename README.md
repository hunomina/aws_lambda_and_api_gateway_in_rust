# Test AWS Lambda + API Gateway with Rust

This lambda function is supposed to be deployed behind an AWS API Gateway (proxy enabled).

The `lambda` module provides all the basic structures to :

- Parse received requests from the AWS API Gateway
- Format responses for the AWS API Gateway

Links :

- API Gateway forwarded request format : <https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-lambda-proxy-integrations.html#api-gateway-simple-proxy-for-lambda-input-format>
- API Gateway expected lambda function output format : <https://aws.amazon.com/fr/premiumsupport/knowledge-center/malformed-502-api-gateway/>

Request lifecycle :

1. An HTTP request is sent to the AWS API Gateway
2. The gateway receives and transforms the incoming request (link 1)
3. Send the transformed request to the lambda function
4. The lambda function handles the request and return a response respecting the API Gateway format (link 2)
5. The gateway transforms the received response and send it to the end user
