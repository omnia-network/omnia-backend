# Omnia Backend
Omnia Backend is the core of the Omnia Network and is deployed as a canister on the [Internet Computer](https://internetcomputer.org). It manages the interaction main components of the Omnia Network.

We suggest reading the [Architecture](./docs/architecture.md) document to understand how the Omnia Network works before diving into the code.

## Structure
The Omnia Backend is fully deployed on the Internet Computer and is composed of the following canisters:
- [omnia_backend](./src/omnia_backend), the main canister that exposes the methods to Gateways, Managers and Applications.
- [database](./src/database), that stores all the data (devices, gateways, applications, etc.) using custom Create, Read, Update and Delete (**CRUD**) [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)s structures. 

An **RDF database** is also embedded in the **omnia_backend** canister. See [RDF database](./docs/rdf-database.md) for details.

### Additional libraries
To serve the purposes of the canisters mentioned above, some additional libraries are available in the source code:
- [omnia_utils](./src/omnia_utils) contains some utility functions used by the other canisters.
- [omnia_types](./src/omnia_types) contains the types shared between the canisters.

## Development
We suggest using the Dev Container with Visual Studio Code. If you can't use it (e.g. there are some problems with macOS on M1/M2 chips), have a look at the [Dockerfile](./.devcontainer/Dockerfile) and install the dependencies manually.

### Generate DIDs
`dfx` utility still doesn't support automatic did interfaces generation. There's a workaround, which uses Rust tests and Rust DFINITY CDK under the hood: for each canister, there's a `generate_candid_interface` _test_ function that saves the candid interface in the canister's `.did` file. To run the did generation, run:
```bash
npm run generate:dids
```

**TODO**:
> DIDs describe methods that canisters expose. According to the amazing [Effective Rust Canisters](https://mmapped.blog/posts/01-effective-rust-canisters.html#canister-interfaces) guide, DIDs should be the single source of truth and should be written manually. DIDs should also include documentation (comments) to make it easier to understand what each method does. Hence, **whenever a new method is added to a canister, or an existing method is changed, the corresponding DID should be updated as well**.

### Generate Typescript types

Frontend canisters need Typescript types reflect backend canisters did interfaces and make the `@dfinity/agent-js` library work properly. To generate them simply run:
```bash
npm run generate:types
```

## Run canisters locally

### Start local IC replica

To start a local IC replica, use the following command:
```bash
dfx start [--background] [--clean]
```
where `--background` is optional and makes it run in the background and `--clean` is optional and cleans the replica from previous deployments if there are.

### First deployment

First of all, start a local IC replica (see above).

There are two main canisters in the backend:
- **database**, which stores all data
- **omnia_backend**, which runs all the logics and exposes the methods to the frontend 

As you can guess, **omnia_backend** depends on **database** and needs to know the database canister id to make inter-canister calls work.

So, follow these steps to deploy everything correctly:

1. generate dids and deploy all backend canisters by running:
    ```bash
    npm run generate-dids-and-deploy:backend
    ```

### Deployment

Before executing any of the following commands, make sure you create a `.env` file by copying the `.env.example` file and filling in the parameters.

Available commands:
- `npm run deploy`: deploys **all** canisters
- `npm run deploy:backend`: deploys **backend** canisters only
- `npm run generate-dids-and-deploy`: first generates dids and then deploys **all** canisters
- `npm run generate-dids-and-deploy:backend`: first generates dids and then deploys **backend** canisters only

## Tests (Backend only)

We use [Jest](https://jestjs.io/) to run integration tests. All tests are in the [`__tests__`](./__tests__/) folder.

To run tests, first deploy **backend** canisters (see above for details):
```bash
npm run deploy:backend
```

and then run:
```bash
npm run test
```
After running the tests, it's recommended to stop the local replica and start it again with the `--clean` flag (see above).

## License
Licensed under the [MIT License](./LICENSE).

### Dependencies report
To generate a report of all dependencies licenses, run:
```bash
npm run report:licenses
```
