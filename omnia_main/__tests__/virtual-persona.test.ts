// import { VirtualPersonaValue } from "../src/declarations/omnia_backend/omnia_backend.did";
// import { LONG_TEST_TIMEOUT } from "./utils/constants";
// import {
//   virtualPersona1,
//   virtualPersona1Data,
//   virtualPersona2,
//   virtualPersona2Data,
// } from "./utils/actors";

// // test data
// const ENVIRONMENT_NAME = "test_environment";
// const GATEWAY1_NAME = "test_gateway1";
// const GATEWAY2_NAME = "test_gateway2";
// const TOTAL_GATEWAYS_IN_ENV = 2;
// const DEVICE1_NAME = "test_device1";
// const DEVICE2_NAME = "test_device2";
// const TOTAL_DEVICES_IN_ENV = 2;

// // test variables
// // let newEnvironment: EnvironmentCreationResult;

// describe("Virtual Persona", () => {
//   it("getProfile: any Virtual Persona can get profile data", async () => {
//     const virtualPersona1Actor = await virtualPersona1.getActor();
//     const virtualPersona1Profile = await virtualPersona1.callMethodWithChallenge(
//       async (nonce) => {
//         return virtualPersona1Actor.getProfile(nonce);
//       },
//       virtualPersona1Data.remoteIp
//     );
//     expect(virtualPersona1Profile.error).toBeNull();
//     expect(virtualPersona1Profile.data).toMatchObject<VirtualPersonaValue>({
//       manager_env_uid: [],
//       user_env_uid: [],
//       virtual_persona_principal_id: (await virtualPersona1Data.identity).getPrincipal().toText(),
//       virtual_persona_ip: virtualPersona1Data.remoteIp,
//     });

//     const virtualPersona2Actor = await virtualPersona2.getActor();
//     const virtualPersona2Profile = await virtualPersona2.callMethodWithChallenge(
//       async (nonce) => {
//         return virtualPersona2Actor.getProfile(nonce);
//       },
//       virtualPersona2Data.remoteIp
//     );
//     expect(virtualPersona2Profile.error).toBeNull();
//     expect(virtualPersona2Profile.data).toMatchObject<VirtualPersonaValue>({
//       manager_env_uid: [],
//       user_env_uid: [],
//       virtual_persona_principal_id: (await virtualPersona2Data.identity).getPrincipal().toText(),
//       virtual_persona_ip: virtualPersona2Data.remoteIp,
//     });

//   }, LONG_TEST_TIMEOUT);
// });
