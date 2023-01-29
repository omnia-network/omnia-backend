import path from "path";
import { Actor, ActorSubclass, HttpAgent } from "@dfinity/agent";
// @ts-ignore
import { idlFactory } from "../../src/declarations/omnia_backend/omnia_backend.did.js";
import { _SERVICE } from "../../src/declarations/omnia_backend/omnia_backend.did";

function initCanisterEnv() {
  let localCanisters, prodCanisters;
  try {
    localCanisters = require(path.resolve(
      ".",
      ".dfx",
      "local",
      "canister_ids.json"
    ));
  } catch (error) {
    console.log("No local canister_ids.json found. Continuing production");
  }
  try {
    prodCanisters = require(path.resolve("..", "canister_ids.json"));
  } catch (error) {
    console.log("No production canister_ids.json found. Continuing with local");
  }


  const network =
    process.env.DFX_NETWORK ||
    (process.env.NODE_ENV === "production" ? "ic" : "local");

  process.env.DFX_NETWORK = network;

  const canisterConfig: {
    [key: string]: {
      [key: string]: string;
    },
  } = network === "local" ? localCanisters : prodCanisters;

  const defaultEnv: {
    [key: string]: string;
  } = {
    DFX_NETWORK: network,
  };

  return Object.entries(canisterConfig)
    .reduce((prev, current) => {
      const [canisterName, canisterDetails] = current;
      prev[canisterName.toUpperCase() + "_CANISTER_ID"] =
        canisterDetails[network];
      return prev;
    }, defaultEnv);
};

// we need to recreate the agent because the environment variables are not available at the time of import
const createActor = () => {
  const conf = initCanisterEnv();

  const agent = new HttpAgent({
    host: "http://localhost:4943",
  });

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK !== "ic") {
    agent.fetchRootKey().catch((err) => {
      console.warn(
        "Unable to fetch root key. Check to ensure that your local replica is running"
      );
      console.error(err);
    });
  }

  return Actor.createActor(idlFactory, {
    agent,
    // @ts-ignore
    canisterId: conf.OMNIA_BACKEND_CANISTER_ID,
  }) as ActorSubclass<_SERVICE>;
};

export const omniaApi = createActor();
