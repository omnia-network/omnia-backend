import path from "path";

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

export const canisterEnv = initCanisterEnv();