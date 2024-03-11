<h1 style="text-align: center;">
Cedar Agent
</h1>

[![Current Crates.io Version](https://img.shields.io/crates/v/cedar-agent.svg)](https://crates.io/crates/cedar-agent)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## What is Cedar-Agent?

Cedar-Agent is an HTTP server designed to efficiently manage a policy store and a data store.
It provides a seamless integration with [Cedar](https://www.cedarpolicy.com/en), a language for defining permissions as
policies.  
With Cedar-Agent, you can easily control and monitor access to your application's resources by leveraging Cedar
policies.
If you are not familiar with Cedar, we encourage you to visit the [Cedar website](https://www.cedarpolicy.com/en)
and [playground](https://www.cedarpolicy.com/en/playground) to learn more about it.

Learn more reading these blog posts:
- [Policy as Code: OPA's Rego vs. AWS Cedar](https://www.permit.io/blog/opa-vs-cedar)
- [Open-Sourcing AWS Cedar is a Gamechanger for IAM](https://www.permit.io/blog/oss-aws-cedar-is-a-gamechanger-for-iam)

### Policy Store Management

Cedar-Agent includes a store that allows you to create, retrieve, update, and delete policies.
These policies define who should have access to what resources within your application.
The policy store provides a centralized and flexible way to manage permissions, enabling fine-grained control over user
access.  
Featured Policy Stores :

- [x] In-Memory
- [ ] Redis

### Data Store Management

In addition to the policy store, Cedar-Agent also provides an in-memory data store. This data store allows you to store
and manage your application's data efficiently. By integrating the data store with Cedar-Agent, you can perform
authorized checks on the stored data based on incoming HTTP requests.  
Featured Data Stores :

- [x] In-Memory
- [ ] Redis

### Schema Store Management

Cedar-Agent support storing custom schemas, which hold the shape of your data types and actions. Utilising the schema
store enables you to create a strict definition of all the objects used by your application. Cedar-Agent will validate
all your policies and data against this schema.
Featured Polict Stores :

- [x] In-Memory
- [ ] Redis

### Authorization Checks

One of the key features of Cedar-Agent is its ability to perform authorization checks on stored policies and data.  
By evaluating the Cedar policies, Cedar-Agent ensures that each user's access is restricted to the resources they are
permitted to access.  
Authorization checks are performed based on the incoming HTTP requests, providing an easy-to-use robust and secure
mechanism for controlling access to your application.

Cedar-Agent offers a comprehensive solution for managing policies, data, and authorization checks within your
application. With its seamless integration with Cedar and its robust HTTP server capabilities, Cedar-Agent empowers you
to enforce fine-grained access control and protect your resources effectively.

## How to Use

To use Cedar-Agent, follow the steps below:

### Prerequisites

Before proceeding, ensure that you have Rust and Cargo installed on your system. If you don't have them installed, you
can visit the official [Rust installation page](https://www.rust-lang.org/tools/install) and follow the instructions
specific to your operating system.

### Clone the Repository

Start by cloning the Cedar-Agent repository to your local machine:

```shell
git clone https://github.com/permitio/cedar-agent.git
cd cedar-agent
```

### Build

To build Cedar-Agent, use the following command:

```shell
cargo build
```

### Configuration

Cedar Agent configuration is available using environment variables and command line arguments.

- The port on which the Cedar Agent will listen for incoming HTTP requests. Defaults to `8180`.  
  `CEDAR_AGENT_PORT` environment variable.  
  `--port`, `-p` command line argument.
- Authentication token to enforce using the `Authorization` header. Defaults to `None`.  
  `CEDAR_AGENT_AUTHENTICATION` environment variable.  
  `--authentication`, `-a` command line argument.
- The address of the HTTP server. Defaults to `127.0.0.1`.  
  `CEDAR_AGENT_ADDR` environment variable.  
  `--addr` command line argument.
- The log level to filter logs. Defaults to `info`.  
  `CEDAR_AGENT_LOG_LEVEL` environment variable.  
  `--log-level`, `-l` command line argument.
- Load schema from json file. Defaults to `None`.  
  `CEDAR_AGENT_SCHEMA` environment variable.
  `--schema`, `-s` command line argument.
- Load data from json file. Defaults to `None`.  
  `CEDAR_AGENT_DATA` environment variable.
  `--data`, `-d` command line argument.
- Load policies from json file. Defaults to `None`.
  `CEDAR_AGENT_POLICIES` environment variable.
  `--policies` command line argument.

**command line arguments take precedence over environment variables when configuring the Cedar Agent**

### Run

There are several ways to run the Cedar Agent

#### Run with cargo

To run Cedar-Agent, use the following command:

```shell
cargo run
```

to add any arguments to the command append them after `--`, for example:

```shell
cargo run -- --port 8080
```

#### Run the binary

To run the binary, make sure you've done the [build step](#build), and run this command:

```shell
./target/debug/cedar-agent
```

To check the arguments you can pass to the binary, run:

```shell
./target/debug/cedar-agent --help
```

#### Run with docker

To execute the Cedar Agent docker image, use the following command:

```shell
docker run -p 8180:8180 permitio/cedar-agent
```

### Test

To test Cedar-Agent, use the following command:

```shell
cargo test
```

### API Endpoints

After running Cedar-Agent, the application provides comprehensive API documentation and endpoint schema
using Rapidoc and Swagger UI, that you can access through the following routes:

- http://localhost:8180/rapidoc: Visit this route in your web browser to explore the interactive API
  documentation powered by the Rapidoc tool. It provides detailed information about each endpoint,
  including their parameters,
  request bodies, and response structures.
- http://localhost:8180/swagger-ui: Access this route to interact with the Swagger UI,
  which offers a user-friendly interface to browse the API endpoints.
  It presents a visual representation of the available routes, along with their descriptions,
  request and response schemas, and example requests.

### Quickstart

1. [Run the Cedar Agent](#run)
2. Store schema using this command:

    ```shell
    curl -X PUT -H "Content-Type: application/json" -d @./examples/schema.json http://localhost:8180/v1/schema
    ```

3. Store policy using this command:

    ```shell
    curl -X PUT -H "Content-Type: application/json" -d @./examples/policies.json http://localhost:8180/v1/policies
    ```

4. Store data using this command:

    ```shell
    curl -X PUT -H "Content-Type: application/json" -d @./examples/data.json http://localhost:8180/v1/data
    ```

5. Perform IsAuthorized check using this command:

    ```shell
    curl -X POST -H "Content-Type: application/json" -d @./examples/allowed_authorization_query.json http://localhost:8180/v1/is_authorized
    ```

   The response is:

    ```json
    {
      "decision": "Allow",
      "diagnostics": {
        "reason": [
          "admins-policy"
        ],
        "errors": []
      }
    }
    ```
   As you can see the user is allowed to access the resource because policy id `admins-policy` permits it.  
   Check for a user that is not allowed to access the resource:

    ```shell
   curl -X POST -H "Content-Type: application/json" -d @./examples/denied_authorization_query.json http://localhost:8180/v1/is_authorized
    ```

   The response is:

    ```json
   {
    "decision": "Deny",
    "diagnostics": {
      "reason": [],
      "errors": []
      }
    }
    ```
   As you can see the user is denied access to the resource because no policy allows this request.

**For more details about the performed requests you can check the [examples directory](examples)**

## Run Cedar-agents at scale with OPAL
Want to run multiple Cedar-agents and have them loaded with the data and policeis you need? Try [OPAL](https://github.com/permitio/opal).
OPAL (Open Policy Administration Layer) is a sister project to Cedar-Agent, which has become the de-facto way to manage policy agents (including others like OPA) at scale.
Check out the [tutorial for Cedar+OPAL in the OPAL docs](https://docs.opal.ac/tutorials/cedar).

## Community

Come talk to us about Cedar Agent, or authorization in general - we would love to hear from you ❤️

You can raise questions and ask for features to be added to the road-map in our [**GitHub
discussions**](https://github.com/permitio/cedar-agent/discussions),
report issues in [**GitHub issues**](https://github.com/permitio/cedar-agent/issues),
join our Slack community to chat about authorization, open-source, realtime communication, tech, or anything else!

If you are using our project, please consider giving us a ⭐️

[![Button][join-slack-link]][badge-slack-link]

## Contributing

If you encounter any issues or have suggestions for improvement, please open
an [issue](https://github.com/permitio/cedar-agent/issues), on the Cedar-Agent GitHub repository to get assistance from
the community.

- Pull requests are welcome! (please make sure to include passing tests and docs)
- Prior to submitting a PR - open an issue on GitHub, or make sure your PR addresses an existing issue well.

[join-slack-link]: https://i.ibb.co/wzrGHQL/Group-749.png

[badge-slack-link]: https://io.permit.io/opalcommunity
