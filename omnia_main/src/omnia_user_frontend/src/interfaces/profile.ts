import { VirtualPersona } from "../../../declarations/omnia_backend/omnia_backend.did";

export interface IProfileContext {
  profile: VirtualPersona | null;
  isLoading: boolean;
  login: () => Promise<void>;
  fetchProfileFromCanister: () => Promise<void>;
};
