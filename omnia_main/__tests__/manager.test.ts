// import { VirtualPersonaValue } from "../src/declarations/omnia_backend/omnia_backend.did";
// import { LONG_TEST_TIMEOUT } from "./utils/constants";
// import {
//   manager1,
//   manager1Data,
//   manager2,
//   manager2Data,
// } from "./utils/actors";

// // test variables
// // let newEnvironment: EnvironmentCreationResult;

// describe("Manager", () => {
//   it("getProfile: any Manager can get profile data", async () => {
//     const manager1Actor = await manager1.getActor();
//     const manager1Profile = await manager1.callMethodWithChallenge(
//       async (nonce) => {
//         return manager1Actor.getProfile(nonce);
//       },
//       manager1Data.remoteIp
//     );
//     expect(manager1Profile.error).toBeNull();
//     expect(manager1Profile.data).toMatchObject<VirtualPersonaValue>({
//       manager_env_uid: expect.arrayContaining([expect.any(String)]),
//       user_env_uid: [],
//       virtual_persona_principal_id: (await manager1Data.identity).getPrincipal().toText(),
//       virtual_persona_ip: manager1Data.remoteIp,
//     });

//     const manager2Actor = await manager2.getActor();
//     const manager2Profile = await manager2.callMethodWithChallenge(
//       async (nonce) => {
//         return manager2Actor.getProfile(nonce);
//       },
//       manager2Data.remoteIp,
//     );
//     expect(manager2Profile.error).toBeNull();
//     expect(manager2Profile.data).toMatchObject<VirtualPersonaValue>({
//       manager_env_uid: [],
//       user_env_uid: [],
//       virtual_persona_principal_id: (await manager2Data.identity).getPrincipal().toText(),
//       virtual_persona_ip: manager2Data.remoteIp,
//     });

//   }, LONG_TEST_TIMEOUT);

//   // it();
// });
