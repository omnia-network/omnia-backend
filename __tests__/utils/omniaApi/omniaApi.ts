import path from "path";
import { Actor, ActorSubclass, HttpAgent, Identity } from "@dfinity/agent";
// @ts-ignore
import { idlFactory } from "../../../src/declarations/omnia_backend/omnia_backend.did.js";
import { _SERVICE } from "../../../src/declarations/omnia_backend/omnia_backend.did";
import { OMNIA_BACKEND_CANISTER_ID } from "./canisterEnv";
import { httpNonceChallenge } from "./http";
import { GenericResult, resultParser } from "./resultParser";

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
    canisterId: OMNIA_BACKEND_CANISTER_ID,
  }) as ActorSubclass<_SERVICE>;
};

export class OmniaApi {
  private _identity: Promise<Identity>;
  private _actor: ActorSubclass<_SERVICE> | undefined;

  constructor(identity: Promise<Identity>) {
    this._identity = identity;
  }

  async getActor() {
    return this._actor || await createActor(this._identity);
  }

  async callMethodWithChallenge<T>(
    callback: (nonce: string) => Promise<GenericResult<T>>,
    remoteIp: string,
    proxyData?: { peerId: string },
  ) {
    const nonce = await httpNonceChallenge(remoteIp, proxyData);
    return resultParser<T>(await callback(nonce));
  }

  async parseResult<T>(
    result: Promise<GenericResult<T>>,
  ) {
    return resultParser<T>(await result);
  }
};
