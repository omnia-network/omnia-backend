import { Actor, ActorSubclass, HttpAgent } from "@dfinity/agent";
import { _SERVICE } from "../../src/declarations/application_placeholder/application_placeholder.did";
// @ts-ignore
import { idlFactory } from "../../src/declarations/application_placeholder/application_placeholder.did.js";
import { APPLICATION_PLACEHOLDER_CANISTER_ID } from "./omniaApi/canisterEnv";
import { GenericResult, resultParser } from "./omniaApi/resultParser";

export const createActor = () => {
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

  // Creates an actor with using the candid interface and the HttpAgent
  return Actor.createActor(idlFactory, {
    agent,
    canisterId: APPLICATION_PLACEHOLDER_CANISTER_ID,
  }) as ActorSubclass<_SERVICE>;
};

export class ApplicationApi {
  private _actor: ActorSubclass<_SERVICE> | undefined;

  getActor() {
    return this._actor || createActor();
  }

  async parseResult<T>(
    result: Promise<GenericResult<T>>,
  ) {
    return resultParser<T>(await result);
  }
};
