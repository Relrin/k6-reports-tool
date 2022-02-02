# k6-reports-tool
A CLI tool for managing k6 test results 

This CLI executable was implemented for covering developer needs in saving K6 load test results after the execution. The current implementation relies on the fact that  we store all generated data in the InfluxDB local or remote node. However, as developer as it would be great to export such information for tracking history between test runs and do analytics based on that.

Currently K6 provides some exports in the [PDF format](https://k6.io/docs/cloud/analyzing-results/result-export/), however such functionality expect from a developer or a company to handle a quite [expensive subscription](https://k6.io/pricing/) what can be an issue for small teams.

- [Quick start](#quick-start)
- [Development](#development)
- [License](#license)

## Features

- Minimalistic implementation for extracting load test results in the CSV format from InfluxDB node(s)

## Quick start
For using this CLI tool will need:

1. Download executable file in according to the used operation system from the [releases page](https://github.com/Relrin/k6-reports-tool/releases).

2. Link executable/binary file to your operation system, so you could invoke `k6-reports` everywhere:

    - Linux / Mac OS

      Move the binary file to the `/usr/local/bin` directory and restart the terminal
        ```
        mv ~/Downloads/k6-reports /usr/local/bin
        ```

    - Windows

        1. Right click on the Windows Logo and select the `System` menu item.
        2. Click on the `Advanced System Settings` button.
        3. Click on the `Environment Variables` button.
        4. Select your `PATH` variable and click in the `Edit` button.
        5. Click on the `New` button.
        6. Add the file path to the directory with the `k6-reports` executable.
        7. Click on the `OK` button a couple of times for applying changes.

3. Restart a terminal or a console to reflect the changes and get access to execute `k6-reports` commands.

For more information about the extracted metrics:
- [k6 Metrics](https://k6.io/docs/using-k6/metrics/)
- [k6 Results export](https://k6.io/docs/cloud/analyzing-results/result-export/)

## Development

To start developing will need to have installed latest [K6](https://k6.io/docs/getting-started/installation/), [Rust](https://www.rust-lang.org/learn/get-started) and [Docker](https://docs.docker.com/install/). 

Instead of manually installing all of the used dependencies (e.g. Grafana nodes), will be more than enough to start up a bunch of Docker container with the following command:
```
docker-compose up -d
```

After it, we can interact with the such environment by exposed ports:

| Port       | Service     |
| ---------- | ----------- |
| 3000       | Grafana     |  
| 8086       | InfluxDB    |
| 8888       | Chronograf  |

For running k6 tests and pushing metrics to a InfluxDB node you can use the k6 CLI with command like this one:
```bash
k6 run --out influxdb=http://influxdb:8086/k6 ./simple-k6-test.js
```

For more information about how to implement and run k6 tests you also check the official [k6 documentation](https://k6.io/docs/getting-started/running-k6/).

## License

The k6-reports-tool project is published under BSD license. For more details read the [LICENSE](https://github.com/Relrin/k6-reports-tool/blob/main/LICENSE) file.
