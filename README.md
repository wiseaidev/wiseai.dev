# 💻 WiseaiDev: A Personal Portfolio & Blog

[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Made With Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Made with Yew](https://img.shields.io/badge/Made%20with-Yew-1f425f.svg?logo=ycombinator&logoColor=white)](https://yew.rs/)

[Yew](https://yew.rs/) is a modern Rust framework for building multi-threaded front-end web applications. It aims to provide a productive and pleasant experience for developing front-end applications in Rust, leveraging its safety and performance benefits. By utilizing Yew, we can create interactive and efficient web applications with ease.

## ⚙️ Building and Running

1. Fork/Clone the GitHub repository.

	```bash
	git clone https://github.com/wiseaidev/wiseai.dev
	```

1. Navigate to the application directory.

	```bash
	cd wiseai.dev
	```

1. Run the client:

	```sh
	make run
	```

Navigate to http://localhost:3000 to explore the landing page.

## 🚀 Deploying to CloudFlare

1. Install [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/get-started/):

   To get started with Cloudflare Workers, you'll need to install the Wrangler CLI, which is a powerful tool for managing your deployments. Open up your command-line interface and enter the following command to globally install Wrangler:

   ```sh
   npm i -g wrangler
   ```

1. Login to Cloudflare Account from the CLI:

   To seamlessly interact with your Cloudflare account, you'll need to log in using Wrangler. Run the following command in your terminal:

   ```sh
   wrangler login
   ```

1. Run Your Build Command:

   Before deployment, you need to build the project. Execute the following command to share the app with the world:

   ```sh
   make build
   ```

1. Create a New Deployment:

   Execute the following command to deploy the app with Wrangler Pages:

   ```sh
   wrangler pages deploy dist
   ```

## 🤝 Contributing

We welcome contributions to enhance the wiseai.dev repository! To contribute, please follow the [`CONTRIBUTING.md`](CONTRIBUTING.md) file guidelines. Thank you for helping make this project better!

## 📜 License

This project and the accompanying materials are made available under the terms and conditions of the [`MIT License`](https://github.com/wiseaidev/wiseai.dev/blob/main/LICENSE).
