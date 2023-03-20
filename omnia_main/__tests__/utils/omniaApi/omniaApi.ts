import path from "path";
import { Actor, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
// @ts-ignore
import { idlFactory } from "../../../src/declarations/omnia_backend/omnia_backend.did.js";
import { _SERVICE } from "../../../src/declarations/omnia_backend/omnia_backend.did";
import { canisterEnv } from "./canisterEnv";
import { httpNonceChallenge } from "./http";

// we need to recreate the agent because the environment variables are not available at the time of import
const createActor = async (identity: Promise<Identity>) => {
  const agent = new HttpAgent({
    host: "http://localhost:4943",
    identity,
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
    canisterId: canisterEnv.OMNIA_BACKEND_CANISTER_ID,
  }) as ActorSubclass<_SERVICE>;
};

export const omniaApi = createActor;

// TODO: move this into a package, since it's needed by Gateways, frontends, etc.
export const callMethodWithChallenge = async (actor: ActorSubclass<_SERVICE>, method: Exclude<keyof _SERVICE, 'http_request' | 'http_request_update'>, remoteIp: string, ...args: any[]) => {
  const nonce = await httpNonceChallenge(remoteIp);
  // @ts-ignore
  return actor[method].apply(null, [nonce, ...args]);
};
