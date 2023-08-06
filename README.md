# 🔥 Rhea - A Server Load Testing Tool

Rhea is a powerful and easy-to-use command-line tool written in Rust for load testing servers. It allows you to simulate heavy concurrent traffic and analyze how your server responds under various loads. With Rhea, you can identify performance bottlenecks, test the scalability of your server, and ensure its stability even in high-traffic scenarios.

## Features

- **Concurrent Requests**: Rhea generates multiple concurrent requests to stress-test your server efficiently.

- **Customizable Load**: You can define the number of concurrent users and the total number of requests to create different load scenarios.

- **HTTP Method Support**: Perform load testing with various HTTP methods, including GET, POST, PUT, DELETE, and more.

- **Open Source**: Rhea is open source, licensed under the MIT License. Feel free to contribute to its development!

## Installation

To install Rhea, you need to have Rust installed. Once you have Rust set up, you can install Rhea via Cargo, the Rust package manager:

```bash
$ cargo install rhea
```

## Quick Start

1. Ensure your server is running and accessible.

2. Open a terminal window and run the following command:

```bash
$ rhea --url http://your-server.com/api --concurrency 100 --requests 1000
```

3. Sit back and relax while Rhea performs the load test.

4. Once the load test is complete, Rhea will display detailed metrics and statistics.

## Usage

```bash
rhea [OPTIONS] --url <URL> --requests <REQ_COUNT>
```

**Flags:**

- `--help`: Displays help information about the load testing tool.

**Options:**

- `--url <URL>`: The URL of the server you want to load test (required).

- `--concurrency <CONCURRENCY>`: The number of concurrent users to simulate (default: 100).

- `--requests <REQUESTS>`: The total number of requests to send during the test (default: 1000).

- `--method <METHOD>`: The HTTP method to use for the requests (default: GET).

- `--body <BODY>`: The request body for POST or PUT requests.

- `--headers <HEADERS>`: Custom headers to include in the requests.

## Contributing

We welcome contributions to Rhea! To contribute, please follow the guidelines outlined in the CONTRIBUTING.md file.

## License

Rhea is open-source software licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgments

We would like to express our gratitude to all the contributors who have made this project possible.

---

Give Rhea a try, and let us know your feedback! If you encounter any issues or have ideas for improvements, feel free to open an issue on GitHub.

Happy load testing!

```
