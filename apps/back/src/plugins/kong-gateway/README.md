# Configuring Kong API Gateway

## Setup the Gateway

The Moesif Developer Portal can be used with a running instance of Kong. To integrate Moesif and Kong, you can follow our guide that covers [integrating Moesif and Kong in detail](https://www.moesif.com/docs/guides/guide-kong-gateway-integration/). Alternatively, you can also check out [our integration documentation for Kong](https://www.moesif.com/docs/server-integration/kong-api-gateway/) if you’re already an experienced Kong user. Once you have the integration set, you’ll be able to complete the rest of the Kong setup for the Developer Portal.

## Configure the Gateway

### Create an Endpoint in Kong Gateway

To monetize your APIs using the developer portal, you’ll need to set up an endpoint in Kong. This allows Kong to route and manage traffic to your upstream API, enabling features like authentication and rate limiting. If you already have an endpoint, you can use the existing one, provided it meets the requirements for your intended use case. Below are the steps to create a new service and route using Kong's CLI.

#### Create a Service

A service in Kong represents your upstream API or microservice. Use the following command to create a service:

```bash
curl -i -X POST http://localhost:8001/services \
    --data name=TestService \
    --data url=https://www.httpbin.org
```

In this command:

- Replace `http://localhost:8001/services` with URL of your Kong admin instance.
- Replace `TestService` with the desired name for your service.
- Replace `https://www.httpbin.org` with the URL of your upstream service.

#### Create a Route

A route specifies how Kong should route requests to your service. To create a route for the service created in Step 1, use the following command:

```bash
curl -i -X POST http://localhost:8001/routes \
    --data service.name=TestService \
    --data name=TestRoute \
    --data methods=GET \
    --data paths=/test-route
```

In this command:

- Replace `http://localhost:8001/routes` with URL of your Kong admin instance.
- Replace `TestService` with the name of your previously created service.
- Replace `TestRoute` with the desired name for the route.
- Replace `/test-route` with the desired path for the route.

#### Verify the Endpoint

Once the service and route are created, you can test the endpoint to ensure it is configured correctly. Use the following command to test the route:

```bash
curl -i http://localhost:8000/test-route
```

In this command:

- Replace `http://localhost:8000/test-route` with URL of your Kong instance.

This command sends a GET request to the route. If everything is configured correctly, you should see a response from your upstream service (`https://www.httpbin.org` in this example).

You should see a `200 OK` response as well as a response body containing the response's contents (essentially a webpage). With our endpoint working, now let’s move on to securing it with an API key.

### Configure Gateway Authentication

#### Adding Key Auth to All Endpoints

Since the Developer Portal generates API keys, you must add and enable the **Key-Auth** plugin to your Kong endpoint. For simplicity, you can enable this plugin globally. If you want to apply **Key-Auth** only to specific/monetized routes, you can do that as well.

To add the Key-Auth plugin globally using the CLI, use the following command:

```bash
curl -i -X POST http://localhost:8001/plugins \
    --data name=key-auth \
    --data config.key_names=apikey \
    --data enabled=true
```

In this command:

- `name=key-auth` specifies the Key-Auth plugin.
- `config.key_names=apikey` sets the header field name to `apikey`.
- `enabled=true` ensures the plugin is active.

To test it out, resend the request from earlier. You should get a `401 Unauthorized` response with a message body stating `No API key found in request`. If you are not getting this response, please refer to the [Kong documentation for key-auth](https://docs.konghq.com/hub/kong-inc/key-auth/).

## Configure the Developer Portal

### Integrating Kong with Moesif

The Moesif-Kong plugin simplifies sending API analytics to Moesif. To set it up, refer to [our integration documentation](https://docs.konghq.com/hub/moesif/kong-plugin-moesif/) or follow the detailed steps in [our integration guide](https://www.moesif.com/docs/guides/guide-kong-gateway-integration/).

After enabling the Moesif-Kong integration you should start seeing API call metrics in Moesif. To secure your API, ensure that unauthenticated calls are blocked by the **key-auth** plugin in Kong. Unauthorized requests should return `401 Unauthorized` responses, which will also appear in Moesif for monitoring.

### Updating Environment Variables

Depending on your setup, variables can be added to either the `my-dev-portal-api/.env` file (for Node setups) or the `distribution/docker-compose.yml` file (for Docker setups).

This should point to the Kong admin API URL and port, not the gateway URL/port used for traffic proxying. If you're running Kong locally, the default admin API URL is `http://localhost:8001`. For remote or custom configurations, update this value to match the correct URL and port.

#### Environment Variables for Node

- Open the `my-dev-portal-api/.env` file.
- Replace the following lines with your admin API URL:

```shell
PLUGIN_APIM_PROVIDER="Kong"
PLUGIN_KONG_URL="http://localhost:8001"
```

- Save the `.env` file to ensure the updated values are persisted.

#### Environment Variables for Docker

- Open the `distribution/docker-compose.yml` file.
- Add or update the following entries in the relevant service configuration under `environment`, replacing with your admin API URL:

```yaml
dev-portal-api:
    environment:
        - PLUGIN_APIM_PROVIDER=Kong
        - PLUGIN_KONG_URL="http://localhost:8001"
```

- Save the `docker-compose.yml` file to ensure the updated values are persisted.

## Testing the Developer Portal

Once the Developer portal is configured, testing out all of the moving parts of the Developer Portal is crucial. Doing this ensures that everything is working as intended. See our detailed testing process [here](https://www.moesif.com/docs/developer-portal/using-the-portal/).

## Verifying Key Provisioning Functionality via Kong CLI

After completing the developer portal configuration, you can verify Kong functionality and key provisioning using the Kong CLI. Follow these steps:

### List All Consumers

Use the following command to fetch the list of all consumers in your Kong instance:

```bash
curl -X GET http://localhost:8001/consumers
```

Replace `http://localhost:8001` with your Kong admin API URL if it's running remotely.

Check the response for your newly created user. The output should include details such as `username`, `id`, and `custom_id`.

### Search for a Specific Consumer by Stripe Customer ID

If you know the `custom_id` (e.g., the Stripe customer ID `stripe_customer_ID`), filter the results to locate the specific consumer entry:

```bash
curl -X GET "http://localhost:8001/consumers?custom_id=stripe_customer_ID"
```

### Retrieve Consumer Details

To view detailed information about the consumer, including associated keys, use the `id` or `username` of the consumer:

```bash
curl -X GET http://localhost:8001/consumers/{consumer_id_or_username}
```

Replace `{consumer_id_or_username}` with the appropriate consumer ID or username.

### Verify the `custom_id` Field

Ensure that the consumer entry includes the `custom_id` field with the Stripe customer ID (e.g., `stripe_customer_ID`). This confirms that the user is successfully added, and key provisioning is functioning correctly.
