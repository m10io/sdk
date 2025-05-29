import type { AxiosResponse } from "axios";
import axios from "axios";
import qs from "qs";
import superagent from "superagent";

export const getBaseAccessToken =  async (address: string, username: string, password: string): Promise<string>  => {
    const url = `${address}/oauth/token`;

    const response: superagent.Response = await superagent
        .post(url)
        .send({
            "client_id": "directory",
            "grant_type": "password",
            "username": username,
            "password": password,
            "audience": "https://api.m10.net",
            "scope": "offline_access openid",
        });

    if (!response?.body?.access_token) {
        throw new TypeError("Access token is None");
    }

    return response?.body?.access_token;
};

const MILLIS_PER_SECOND = 1000;

interface FisAccessTokenResponse {
    access_token: string;
    id_token: string;
    token_type: string;
    expires_in: number;
}

export const getFISAccessToken =  async (
    url: string,
    clientId: string,
    clientSecret: string,
): Promise<{
    accessToken: string;
    accessTokenExpiry: number;
}> => {
    const startTimestamp = Date.now();
    const response: AxiosResponse<FisAccessTokenResponse> = await axios.post(
        `https://${url}/accesstoken`,
        qs.stringify({
            client_id: clientId,
            client_secret: clientSecret,
            grant_type: "client_credentials",
            scope: "openid",
        }),
        {
            headers: { "Content-Type": "application/x-www-form-urlencoded" },
        },
    );

    const data = response.data;
    if (!data) {
        throw new Error("access token response data was null");
    }

    const accessToken = data.access_token;
    const accessTokenExpiry = startTimestamp + (data.expires_in * MILLIS_PER_SECOND);

    if (typeof accessToken !== "string" || typeof accessTokenExpiry !== "number") {
        throw new Error("access token could not be parsed from response");
    }

    return { accessToken, accessTokenExpiry };
};
