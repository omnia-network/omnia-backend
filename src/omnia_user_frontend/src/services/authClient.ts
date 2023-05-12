import { AuthClient } from "@dfinity/auth-client";

export const getAuthClient = async () => {
    return await AuthClient.create();
}