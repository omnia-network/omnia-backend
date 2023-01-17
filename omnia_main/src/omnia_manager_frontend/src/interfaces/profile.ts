import { UserProfile } from "../../../declarations/omnia_backend/omnia_backend.did";

export interface IProfileContext {
  profile: UserProfile | null;
  isLoading: boolean;
  login: () => Promise<void>;
  fetchProfileFromCanister: () => Promise<void>;
};
