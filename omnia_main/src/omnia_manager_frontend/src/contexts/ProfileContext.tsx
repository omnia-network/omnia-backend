import React, { createContext, useEffect, useState, useContext } from "react";
import { omnia_backend } from "../../../declarations/omnia_backend";
import DataView from "../components/DataView";
import { IProfileContext } from "../interfaces/profile";
import { getAuthClient } from "../services/authClient";


const ProfileContext = createContext<IProfileContext>({
  profile: null,
  isLoading: true,
  login: async () => { },
  fetchProfileFromCanister: async () => { }
});

type IProps = {
  children?: React.ReactNode
};

export const ProfileProvider: React.FC<IProps> = ({ children }) => {
  const [profile, setProfile] = useState<IProfileContext["profile"] | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  const fetchProfileFromCanister = async () => {
    const profile = await omnia_backend.getProfile();
    console.log("Profile from canister", profile);
    setProfile(profile);
  };

  const login = async () => {
    setIsLoading(true);

    const authClient = await getAuthClient();
    authClient.login({
      async onSuccess() {
        console.log("Login success");
        // get user profile from canister
        await fetchProfileFromCanister();

        setIsLoading(false);
      },
      onError(error) {
        console.log(error);

        setIsLoading(false);
      },
      identityProvider: `http://localhost:4943?canisterId=${process.env.INTERNET_IDENTITY_CANISTER_ID}`
    });
  };

  useEffect(() => {
    getAuthClient().then(async (authClient) => {
      console.log("Auth Client initialized", authClient);
      const isAuth = await authClient.isAuthenticated();
      if (isAuth) {
        console.log("User is logged in, getting profile from canister");
        await fetchProfileFromCanister();
      }
      setIsLoading(false);
    }).catch((e) => {
      console.log("Error initializing auth client", e);
      setIsLoading(false);
    });
  }, []);

  return (
    <ProfileContext.Provider value={{ profile, isLoading, login, fetchProfileFromCanister }}>
      {isLoading
        ? (
          <div style={{ marginTop: 20 }}>Loading...</div>
        )
        : (
          !profile
            ? (
              <div style={{ marginTop: 20 }}>
                <button onClick={login}>Login</button>
              </div>
            )
            : <ProfileWrapper>{children}</ProfileWrapper>
        )
      }
    </ProfileContext.Provider>
  );
};

export default ProfileContext;

type IProfileWrapperProps = {
  children?: React.ReactNode
};

const ProfileWrapper: React.FC<IProfileWrapperProps> = ({ children }) => {
  const { profile } = useContext(ProfileContext);

  return (
    <div style={{
      display: "flex",
      flexDirection: "row",
      justifyContent: "start",
      alignItems: "start",
      padding: 20,
      height: "100vh",
    }}>
      <div style={{
        width: "25%",
        borderRight: "1px solid #ccc",
        height: "100%",
        textAlign: "left",
      }}>
        <h2>Your Omnia Profile</h2>
        <DataView
          data={profile}
        />
      </div>
      <div style={{ width: "75%"}}>
        {children}
      </div>
    </div>
  );
};
