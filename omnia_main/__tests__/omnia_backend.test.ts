import { validate } from "uuid";
import { VirtualPersonaValue } from "../src/declarations/omnia_backend/omnia_backend.did";
import { manager1 } from "./utils/actors";
import { callMethodWithChallenge, omniaApi } from "./utils/omniaApi/omniaApi";

// test timeouts
const LONG_TEST_TIMEOUT = 30_000;

// test data
const ENVIRONMENT_NAME = "test_environment";
const GATEWAY1_NAME = "test_gateway1";
const GATEWAY2_NAME = "test_gateway2";
const TOTAL_GATEWAYS_IN_ENV = 2;
const DEVICE1_NAME = "test_device1";
const DEVICE2_NAME = "test_device2";
const TOTAL_DEVICES_IN_ENV = 2;

// test variables
// let newEnvironment: EnvironmentCreationResult;

describe("Profile", () => {
  it("getProfile: anyone can get profile data", async () => {

    const manager1Actor = await omniaApi(manager1.identity);

    const profile = await callMethodWithChallenge(
      manager1Actor,
      "getProfile",
      manager1.remoteIp,
    );

    // @ts-ignore
    expect(profile["Err"]).toBeUndefined();
    // @ts-ignore
    expect(profile["Ok"]).toMatchObject<VirtualPersonaValue>({
      manager_env_uid: [],
      user_env_uid: [],
      virtual_persona_principal_id: (await manager1.identity).getPrincipal().toText(),
      virtual_persona_ip: manager1.remoteIp,
    });
  }, LONG_TEST_TIMEOUT);
});
